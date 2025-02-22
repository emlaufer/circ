//! Non-cryptographic pre-computation.
//!
//! Conceptually, this machinery allows a party with input material for one computation to map it
//! into input material for another computation.

use fxhash::{FxHashMap, FxHashSet};

use crate::ir::term::*;

/// A "precomputation".
///
/// Expresses a computation to be run in advance by a single party.
/// This may be "multi-epoched", meaning some random coins must be resolved
/// before other values can be computed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PreComp {
    /// A map from output names to the terms that compute them.
    outputs: FxHashMap<String, Term>,
    /// The order that precomputes must be resolved in.
    pub sequence: Vec<String>,
}

impl PreComp {
    /// Create a new precomputation
    pub fn new() -> Self {
        Self::default()
    }
    /// immutable access to the outputs
    pub fn outputs(&self) -> &FxHashMap<String, Term> {
        &self.outputs
    }
    /// Add a new output variable to the precomputation. `value` is the term that computes its value.
    pub fn add_output(&mut self, name: String, value: Term) {
        self.sequence.push(name.clone());
        let old = self.outputs.insert(name, value);
        assert!(old.is_none());
    }

    /// Retain only the parts of this precomputation that can be evaluated from
    /// the `known` inputs.
    pub fn restrict_to_inputs(&mut self, mut known: FxHashSet<String>) {
        // TODO: test this
        let os = &mut self.outputs;
        let seq = &mut self.sequence;
        let o_tuple = term(Op::Tuple, os.values().cloned().collect());
        let cache = &mut TermSet::new();
        for t in PostOrderIter::new(o_tuple) {
            if let Op::Var(ref name, _) = &t.op {
                if !known.contains(name) {
                    cache.insert(t);
                }
                continue;
            }

            if t.cs.iter().any(|c| cache.contains(c)) {
                cache.insert(t);
            }
            continue;
        }

        seq.retain(|s| {
            let o = os.get(s).unwrap();
            let drop = cache.contains(o);
            if drop {
                os.remove(s);
            } else {
                known.insert(s.clone());
            }
            !drop
        });
    }
    /// Evaluate the precomputation.
    ///
    /// Requires an input environment that binds all inputs for the underlying computation.
    pub fn eval(&self, env: &FxHashMap<String, Value>) -> FxHashMap<String, Value> {
        let mut value_cache: TermMap<Value> = TermMap::new();
        let mut env = env.clone();
        // iterate over all terms, evaluating them using the cache.
        for o_name in &self.sequence {
            let o = self.outputs.get(o_name).unwrap();
            eval_cached(o, &env, &mut value_cache);
            env.insert(o_name.clone(), value_cache.get(o).unwrap().clone());
        }
        env
    }
    /// Compute the inputs for this precomputation
    pub fn inputs_to_terms(&self) -> FxHashMap<String, Term> {
        PostOrderIter::new(term(Op::Tuple, self.outputs.values().cloned().collect()))
            .filter_map(|t| match &t.op {
                Op::Var(name, _) => Some((name.clone(), t.clone())),
                _ => None,
            })
            .collect()
    }

    /// Compute the inputs for this precomputation
    pub fn inputs(&self) -> FxHashSet<String> {
        self.inputs_to_terms().into_keys().collect()
    }

    /// Bind the outputs of `self` to the inputs of `other`.
    pub fn sequential_compose(mut self, other: &PreComp) -> PreComp {
        for o_name in &other.sequence {
            let o = other.outputs.get(o_name).unwrap().clone();
            assert!(!self.outputs.contains_key(o_name));
            self.outputs.insert(o_name.clone(), o);
            self.sequence.push(o_name.clone());
        }
        self
    }
}
