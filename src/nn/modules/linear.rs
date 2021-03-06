use nn::{Module, ModuleStruct, ModIntf, Parameter};
use autograd::Variable;
use std::marker::PhantomData;


#[derive(ModParse)]
pub struct Linear<T> {
    delegate: Module<T>,
    #[ignore]
    in_features: u32,
    #[ignore]
    out_features: u32,
    weight: Parameter<T>,
    bias: Option<Parameter<T>>,
}

impl<T: Default> Linear<T> {
    pub fn build(in_features: u32, out_features: u32) -> LinearArgsBuilder<T> {
        LinearArgsBuilder::default()
            .in_features(in_features)
            .out_features(out_features)
    }
    pub fn new(args: LinearArgs<T>) -> Linear<T> {
        let mut t = Linear {
            delegate: Module::new(),
            in_features: args.in_features,
            out_features: args.out_features,
            weight: Parameter::default(),
            bias: None,
        };
        t.init_module();
        t
    }
}
#[builder(pattern="owned")]
#[derive(Builder)]
pub struct LinearArgs<T: Default> {
    in_features: u32,
    out_features: u32,
    #[builder(default="true")]
    bias: bool,
    #[builder(default="PhantomData")]
    phantom: PhantomData<T>,
}
impl<T: Default> LinearArgsBuilder<T> {
    pub fn done(self) -> Linear<T> {
        let args = self.build().unwrap();
        Linear::new(args)
    }
}
impl<T: Default> ModIntf<T> for Linear<T> {
    fn delegate(&mut self) -> &mut Module<T> {
        &mut self.delegate
    }
    fn forward(&mut self, input: &mut Variable<T>) -> Variable<T> {
        input.clone()
    }
    fn forwardv(&mut self, input: &mut Vec<Variable<T>>) -> Vec<Variable<T>> {
        panic!("not valid");
    }
}
