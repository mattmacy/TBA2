use std::collections::HashMap;
use std::collections::hash_map;
use tensor::*;
use nn::Parameter;


// placeholder
struct TorchBackend {}

pub trait ModuleStruct<'a> {
    fn init_module(&mut self);
}

pub struct Module<'a> {
    pub _name: &'a str,
    _backend: TorchBackend,

   	_buffers: HashMap<&'a str, &'a mut Tensor>,
    //	_backward_hooks:
    //	_forward_hooks:
    _params: HashMap<&'a str, *mut Parameter<'a>>,
    _modules: HashMap<&'a str, *mut Module<'a>>,
    training: bool,
}
pub struct PtrIterMut<'a, T: 'a> {
    //mod_iter: linked_hash_map::IterMut<'a, &'a str, *mut T>,
    mod_iter: hash_map::IterMut<'a, &'a str, *mut T>,
}
pub struct PtrIter<'a, T: 'a> {
    //mod_iter: linked_hash_map::IterMut<'a, &'a str, *mut T>,
    mod_iter: hash_map::Iter<'a, &'a str, *mut T>,
}

impl<'a, T> Iterator for PtrIterMut<'a, T> {
    type Item = (&'a str, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((name, t)) = self.mod_iter.next() {
            Some((name, unsafe { &mut **t as &mut T }))
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for PtrIter<'a, T> {
    type Item = (&'a str, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((name, t)) = self.mod_iter.next() {
            Some((name, unsafe { & **t as & T }))
        } else {
            None
        }
    }
}

impl<'a> Module<'a> {
    pub fn new() -> Module<'a> {
        unsafe { Module {
            _name: "",
            _backend: TorchBackend {},
            _buffers: HashMap::new(),
            _params: HashMap::new(),
            _modules: HashMap::new(),
            training: true,
        }
    }
    }
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Module<'a> {
        self as *mut Module<'a>
    }
    pub fn add_module(&mut self, module: &mut ModIntf<'a>) {
        let m = module.delegate();
        self._modules.insert(m._name, m.as_mut_ptr());

    }
    pub fn modules_iter_mut(&mut self) -> PtrIterMut<Module<'a>> {
        PtrIterMut { mod_iter: self._modules.iter_mut() }
    }
    pub fn modules_iter(&mut self) -> PtrIter<Module<'a>> {
        PtrIter { mod_iter: self._modules.iter() }
    }
    pub fn params_iter_mut(&mut self) -> PtrIterMut<Parameter<'a>> {
        PtrIterMut { mod_iter: self._params.iter_mut() }
    }
    pub fn register_buffer(&mut self, name: &'a str, tensor: &'a mut Tensor) {
        self._buffers.insert(name, tensor);
    }
    fn _apply(&mut self, callback: fn(&mut Tensor)) {
        for (_, module) in self.modules_iter_mut() {
            module._apply(callback)
        }
        for (_, param) in self.params_iter_mut() {
        	param.apply(callback)
            /*
			if let Some(g) = p._grad {
					callback(g.data)
			}
			*/
            /* see also _buffers */
        }
    }
    fn apply(&mut self, callback: fn(&mut Self)) {
        for (_, module) in self.modules_iter_mut() {
            module.apply(callback)
        }
        callback(self)
    }
    pub fn repr(&mut self) -> String {
        let mut tmpstr = format!("{} (\n", self._name );
        for (key, module) in self.modules_iter_mut() {
            let modstr = module.repr();
            let modstr = format!("  {}", modstr);
            tmpstr = format!("{} ({}): {}\n", tmpstr, key, modstr);
        }
        tmpstr = format!("{})", tmpstr);
        tmpstr
    }
    pub fn eval(&mut self) {
        self.train(false)
    }
    fn train(&mut self, mode: bool) {
        self.training = mode;
        for (_, module) in self.modules_iter_mut() {
            module.train(mode)
        }

    }
}

pub trait ModIntf<'a> {
    fn delegate(&mut self) -> &mut Module<'a>;
    fn forward(&mut self);
}