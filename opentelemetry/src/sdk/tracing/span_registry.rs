//use crate::{
//    api::{Span, trace::span::Current, SpanRegistry},
//};
//use sharded_slab::{Guard, Slab};
//use std::{
//    cell::RefCell,
//    convert::TryInto,
//    sync::atomic::{fence, AtomicUsize, Ordering},
//    sync::RwLock,
//};
//use std::collections::HashSet;
//use crate::api::SpanContext;
//use std::time::SystemTime;
//
//#[derive(Debug)]
//pub struct Registry {
//    spans: Slab<Data>,
//}
//
//thread_local! {
//    static CONTEXT: RefCell<SpanStack> = RefCell::new(SpanStack::new());
//}
//
//struct ContextId {
//    id: u64,
//    duplicate: bool,
//}
//
//pub(crate) struct SpanStack {
//    stack: Vec<ContextId>,
//    ids: HashSet<u64>,
//}
//
//impl SpanStack {
//    pub(crate) fn new() -> Self {
//        SpanStack {
//            stack: vec![],
//            ids: HashSet::new(),
//        }
//    }
//
//    pub(crate) fn push(&mut self, id: u64) {
//        let duplicate = self.ids.contains(&id);
//        if !duplicate {
//            self.ids.insert(id.clone());
//        }
//        self.stack.push(ContextId { id, duplicate })
//    }
//
//    pub(crate) fn pop(&mut self, expected_id: &u64) -> Option<u64> {
//        if &self.stack.last()?.id == expected_id {
//            let ContextId { id, duplicate } = self.stack.pop()?;
//            if !duplicate {
//                self.ids.remove(&id);
//            }
//            Some(id)
//        } else {
//            None
//        }
//    }
//
//    #[inline]
//    pub(crate) fn current(&self) -> Option<u64> {
//        self.stack
//            .iter()
//            .rev()
//            .find(|context_id| !context_id.duplicate)
//            .map(|context_id| context_id.id)
//    }
//}
//
//#[derive(Debug)]
//pub struct Data {
//    name: &'static str,
//    context: SpanContext,
//    parent: Option<u64>,
//    children: Vec<u64>,
//    ref_count: AtomicUsize,
//}
//
//// === impl Registry ===
//
//impl Default for Registry {
//    fn default() -> Self {
//        Self { spans: Slab::new() }
//    }
//}
//
//#[inline]
//fn idx_to_id(idx: usize) -> u64 {
//    idx as u64 + 1
//}
//
//#[inline]
//fn id_to_idx(id: u64) -> usize {
//    id as usize - 1
//}
//
//impl<'a> SpanRegistry<'a> for Registry {
//    type Span = sdk::Span;
//
//    fn insert(&self, s: Data) -> Option<usize> {
//        self.spans.insert(s)
//    }
//
//    fn get(&self, id: u64) -> Option<Guard<'_, Data>> {
//        self.spans.get(id_to_idx(id))
//    }
//
//    fn get_span_context(&'a self, id: u64) -> Option<Self::Span> {
//        self.get(id)
//    }
//}
//
////impl LookupMetadata for Registry {
////    fn metadata(&self, id: u64) -> Option<&'static str> {
////        if let Some(span) = self.get(id) {
////            Some(span.metadata())
////        } else {
////            None
////        }
////    }
////}
//
//// === impl Data ===
//
//impl Data {
//    pub fn name(&self) -> &'static str {
//        self.name
//    }
//
//    pub fn context(&self) -> &SpanContext {
//        &self.context
//    }
//
//    #[inline(always)]
//    fn with_parent<'registry, F, E>(
//        &self,
//        my_id: u64,
//        last_id: Option<u64>,
//        f: &mut F,
//        registry: &'registry Registry,
//    ) -> Result<(), E>
//        where
//            F: FnMut(u64, Guard<'_, Data>) -> Result<(), E>,
//    {
//        if let Some(span) = registry.get(my_id) {
//            if let Some(parent_id) = span.parent {
//                if Some(parent_id) != last_id {
//                    if let Some(parent) = registry.get(parent_id) {
//                        parent.with_parent(parent_id, Some(my_id), f, registry)?;
//                    } else {
//                        panic!("missing span for {:?}; this is a bug", parent_id);
//                    }
//                }
//            }
//            if let Some(span) = registry.get(my_id) {
//                f(my_id, span);
//            }
//        }
//        Ok(())
//    }
//}
//
//impl Drop for Data {
//    fn drop(&mut self) {
//        // We have to actually unpack the option inside the `get_default`
//        // closure, since it is a `FnMut`, but testing that there _is_ a value
//        // here lets us avoid the thread-local access if we don't need the
//        // dispatcher at all.
//        if self.parent.is_some() {
////            dispatcher::get_default(|subscriber| {
////                if let Some(parent) = self.parent.take() {
////                    let _ = subscriber.try_close(parent);
////                }
////            })
//        }
//    }
//}
//
//impl<'a> Span<'a> for Guard<'a, Data> {
//    type Children = std::slice::Iter<'a, u64>; // not yet implemented...
//    type Follows = std::slice::Iter<'a, u64>;
//
//    fn id(&self) -> u64 {
//        self.idx().try_into().unwrap()
//    }
//    fn parent(&self) -> Option<u64> {
//        self.parent
//    }
//    fn children(&'a self) -> Self::Children {
//        self.children.iter()
//    }
//    fn follows_from(&self) -> Self::Follows {
//        unimplemented!("david: add this to `BigSpan`")
//    }
//    fn add_event(&mut self, message: String) {
//        unimplemented!()
//    }
//
//    fn add_event_with_timestamp(&mut self, message: String, timestamp: SystemTime) {
//        unimplemented!()
//    }
//
//    fn get_context(&'a self) -> &'a SpanContext {
//        &self.context
//    }
//
//    fn is_recording(&self) -> bool {
//        unimplemented!()
//    }
//
//    fn set_attribute(&mut self, key: String, value: String) {
//        unimplemented!()
//    }
//
//    fn end(&mut self) {
//        unimplemented!()
//    }
//}
