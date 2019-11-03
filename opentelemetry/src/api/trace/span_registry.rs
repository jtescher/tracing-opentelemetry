//use crate::api;
//use sharded_slab::Guard;
//
////pub trait LookupMetadata {
////    fn metadata(&self, id: u64) -> Option<&'static Metadata<'static>>;
////    fn exists(&self, id: u64) -> bool {
////        self.metadata(id).is_some()
////    }
////}
//
//pub trait SpanRegistry<'a> {
//    type Span: api::Span<'a>;
//    fn insert(&self, s: Self::Span) -> Option<usize>;
//
//    fn get(&self, id: u64) -> Option<Guard<'a, Self::Span>>;
//
//    fn get_span_context(&'a self, id: u64) -> Option<api::SpanContext>;
//
//    fn span(&'a self, id: u64) -> Option<SpanRef<'a, Self>>
//        where
//            Self: Sized,
//    {
//        let data = self.get_span_context(id)?;
//        Some(SpanRef {
//            registry: self,
//            context: data,
//        })
//    }
//}
//
//#[derive(Debug)]
//pub struct SpanRef<'a, R: SpanRegistry<'a>> {
//    registry: &'a R,
//    context: api::SpanContext,
//}
//
//#[derive(Debug)]
//pub struct Parents<'a, R> {
//    registry: &'a R,
//    next: Option<u64>,
//}
//
//impl<'a, R> SpanRef<'a, R>
//    where
//        R: SpanRegistry<'a>,
//{
//    pub fn id(&self) -> u64 {
//        self.context.id()
//    }
//
//    pub fn parent_id(&self) -> Option<u64> {
//        self.context.parent()
//    }
//
//    pub fn parent(&self) -> Option<Self> {
//        let id = self.context.parent()?;
//        let data = self.registry.get(id)?;
//        Some(Self {
//            registry: self.registry,
//            context: data,
//        })
//    }
//
//    pub fn parents(&'a self) -> Parents<'_, R> {
//        Parents {
//            registry: self.registry,
//            next: self.parent().map(|parent| parent.id()),
//        }
//    }
//
//    pub fn child_ids(&'a self) -> <R::Span as api::Span<'a>>::Children {
//        self.context.children()
//    }
//}
//
//impl<'a, R> Iterator for Parents<'a, R>
//    where
//        R: SpanRegistry<'a>,
//{
//    type Item = SpanRef<'a, R>;
//    fn next(&mut self) -> Option<Self::Item> {
//        let id = self.next.take()?;
//        let span = self.registry.span(id)?;
//        self.next = span.parent().map(|parent| parent.id());
//        Some(span)
//    }
//}
