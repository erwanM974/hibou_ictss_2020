/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::*;
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;

use crate::process::verdicts::CoverageVerdict;
use crate::process::hibou_process::*;
use crate::process::process_manager::*;

pub fn explore(interaction : Interaction,
               gen_ctx : GeneralContext,
               pre_filters : Vec<HibouPreFilter>,
               strategy : HibouSearchStrategy,
               prioritize_action : PrioritizeActionKind,
               loggers : Vec<Box<dyn ProcessLogger>>) {
    // ***
    let mut manager = HibouProcessManager::new(gen_ctx,
                                               strategy,
                                               None,
                                               pre_filters,
                                               HashMap::new(),
                                               ProcessQueue::new(),
                                               prioritize_action,
                                               loggers);
    // ***
    manager.init_loggers(&interaction,&None);
    // ***
    let mut next_state_id : u32 = 1;
    let mut node_counter : u32 = 0;
    enqueue_next_node_in_exploration(&mut manager,next_state_id,interaction,0,0);
    next_state_id = next_state_id + 1;
    node_counter = node_counter +1;
    // ***
    // ***
    while let Some(next_to_process) = manager.extract_from_queue() {
        let new_state_id = next_state_id;
        next_state_id = next_state_id + 1;
        // ***
        let mut parent_state = manager.get_memorized_state(next_to_process.state_id).unwrap().clone();
        // ***
        match manager.process_next(&parent_state,
                                   &next_to_process,
                                   new_state_id,
                                   node_counter) {
            None => {},
            Some( (new_interaction,new_multi_trace,new_depth,new_loop_depth) ) => {
                node_counter = node_counter + 1;
                enqueue_next_node_in_exploration(&mut manager,
                                                 new_state_id,
                                                 new_interaction,
                                                 new_depth,
                                                 new_loop_depth);
            }
        }
        // ***
        parent_state.remaining_ids_to_process.remove(&next_to_process.id_as_child);
        if parent_state.remaining_ids_to_process.len() == 0 {
            manager.forget_state(next_to_process.state_id);
        } else {
            manager.remember_state(next_to_process.state_id,parent_state);
        }
        // ***
    }
    // ***
    manager.term_loggers(None);
    // ***
}



fn enqueue_next_node_in_exploration(manager      : &mut HibouProcessManager,
                                     state_id    : u32,
                                     interaction : Interaction,
                                     depth       : u32,
                                     loop_depth  : u32) {
    // ***
    let mut next_child_id : u32 = 0;
    // ***
    let mut to_enqueue : Vec<(u32,Position,TraceActionKind)> = Vec::new();
    for front_pos in make_frontier(&interaction) {
        let front_act = interaction.get_sub_interaction(&front_pos).as_leaf();
        next_child_id = next_child_id +1;
        to_enqueue.push( (next_child_id,front_pos,front_act.get_action_kind()) );
    }
    // ***
    if next_child_id > 0 {
        let rem_child_ids : HashSet<u32> = HashSet::from_iter((1..(next_child_id+1)).collect::<Vec<u32>>().iter().cloned() );
        let memo_state = MemorizedState::new(interaction,None,rem_child_ids, loop_depth,depth);
        manager.remember_state( state_id, memo_state );
        manager.enqueue_executions( state_id, to_enqueue );
    }
}



