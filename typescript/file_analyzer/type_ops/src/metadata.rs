use rnode::{Visit, VisitMut, VisitMutWith, VisitWith};
use stc_ts_ast_rnode::RIdent;
use stc_ts_types::Type;

pub struct PreventTupleToArray;

impl VisitMut<Type> for PreventTupleToArray {
    fn visit_mut(&mut self, ty: &mut Type) {
        // TODO: PERF
        ty.normalize_mut();
        ty.metadata_mut().prevent_tuple_to_array = true;

        ty.visit_mut_children_with(self);
    }
}

/// Prevent interop with hygiene.
impl VisitMut<RIdent> for PreventTupleToArray {
    fn visit_mut(&mut self, _: &mut RIdent) {}
}

pub struct PreventComplexSimplification;

impl VisitMut<Type> for PreventComplexSimplification {
    fn visit_mut(&mut self, ty: &mut Type) {
        // TODO: PERF
        ty.normalize_mut();
        ty.metadata_mut().prevent_complex_simplification = true;

        ty.visit_mut_children_with(self);
    }
}

/// Prevent interop with hygiene.
impl VisitMut<RIdent> for PreventComplexSimplification {
    fn visit_mut(&mut self, _: &mut RIdent) {}
}

pub struct TypeFinder {
    found: bool,
    check: fn(&Type) -> bool,
}

impl TypeFinder {
    pub fn find<N>(node: &N, check: fn(&Type) -> bool) -> bool
    where
        N: VisitWith<Self>,
    {
        let mut v = TypeFinder { found: false, check };
        node.visit_with(&mut v);
        v.found
    }
}

impl Visit<Type> for TypeFinder {
    fn visit(&mut self, ty: &Type) {
        if self.found {
            return;
        }

        if (self.check)(ty) {
            self.found = true;
            return;
        }

        ty.visit_children_with(self);
    }
}
