////use std::sync::{RwLockReadGuard, RwLock};
////
////#[derive(Debug)]
////struct Slab {
////    slab: Vec<RwLock<Slot>>,
////}
////
////#[derive(Debug)]
////struct Slot {
////    fields: String,
////    span: State,
////}
////
////pub struct Span<'a> {
////    lock: OwningHandle<RwLockReadGuard<'a, Slab>, RwLockReadGuard<'a, Slot>>,
////}
//
//use crate::api;
//use std::time::SystemTime;
//
//pub struct Span {
//
//}
//
//impl<'a> api::Span<'a> for Span {
//    type Children = std::iter::Empty<&'a u64>;
//    type Follows = std::iter::Empty<&'a u64>;
//
//    fn id(&self) -> u64 {
//        unimplemented!()
//    }
//
//    fn parent(&self) -> Option<u64> {
//        unimplemented!()
//    }
//
//    fn children(&'a self) -> Self::Children {
//        unimplemented!()
//    }
//
//    fn follows_from(&'a self) -> Self::Follows {
//        unimplemented!()
//    }
//
//    fn add_event(&mut self, message: String) {
//        unimplemented!()
//    }
//
//    fn add_event_with_timestamp(&mut self, message: String, timestamp: SystemTime) {
//        unimplemented!()
//    }
//
//    fn get_context(&'a self) -> &'a api::SpanContext {
//        unimplemented!()
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
