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

use crate::core::general_context::GeneralContext;

use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;
use crate::core::syntax::position::*;
use crate::core::trace::{AnalysableMultiTrace,MultiTraceCanal,TraceAction};
use crate::process::log::ProcessLogger;
use crate::core::semantics::frontier::make_frontier;
use crate::core::semantics::execute::execute;
use crate::process::verdicts::CoverageVerdict;




/*
#[derive(Clone, PartialEq, Debug)]
pub struct ProcessStateNode {
    pub state_id : Vec<u32>,
    pub interaction : Interaction,
    pub rem_front_or_match : Vec<Position>,
    pub id_for_next_child : u32,
    pub multitrace : Option<AnalysableMultiTrace>,
    pub previous_loop_instanciations : u32
}*/


#[derive(Clone, PartialEq, Debug)]
pub struct MemorizedState {
    pub interaction : Interaction,
    pub multi_trace : Option<AnalysableMultiTrace>,
    pub remaining_ids_to_process : HashSet<u32>,
    pub previous_loop_instanciations : u32
}

impl MemorizedState {
    pub fn new(interaction : Interaction,
               multi_trace : Option<AnalysableMultiTrace>,
               remaining_ids_to_process : HashSet<u32>,
               previous_loop_instanciations : u32) -> MemorizedState {
        return MemorizedState{interaction,multi_trace,remaining_ids_to_process,previous_loop_instanciations};
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum NextToProcessKind {
    Execute(Position)
}

#[derive(Clone, PartialEq, Debug)]
pub struct NextToProcess {
    pub state_id : Vec<u32>,
    pub id_as_child : u32,
    pub kind : NextToProcessKind
}

impl NextToProcess {
    pub fn new(state_id : Vec<u32>,
               id_as_child : u32,
               kind : NextToProcessKind) -> NextToProcess {
        return NextToProcess{state_id,id_as_child,kind};
    }
}

pub struct ProcessQueue {
    pub high_priority : Vec<NextToProcess>,
    pub medium_priority : Vec<NextToProcess>,
    pub low_priority : Vec<NextToProcess>
}

impl ProcessQueue {
    pub fn new() -> ProcessQueue {
        return ProcessQueue{high_priority:Vec::new(),medium_priority:Vec::new(),low_priority:Vec::new()}
    }

    pub fn insert_item_left(&mut self,node:NextToProcess,priority:u32) {
        if priority >= 1 {
            self.high_priority.insert(0,node);
        } else if priority == 0 {
            self.medium_priority.insert(0,node);
        } else {
            self.low_priority.insert(0,node);
        }
    }

    pub fn insert_item_right(&mut self,node:NextToProcess,priority:u32) {
        if priority >= 1 {
            self.high_priority.push(node);
        } else if priority == 0 {
            self.medium_priority.push(node);
        } else {
            self.low_priority.push(node);
        }
    }

    pub fn get_next(&mut self) -> Option<NextToProcess> {
        if self.high_priority.len() > 0 {
            return Some( self.high_priority.remove(0) );
        } else if self.medium_priority.len() > 0 {
            return Some( self.medium_priority.remove(0) );
        } else if self.low_priority.len() > 0 {
            return Some( self.low_priority.remove(0) );
        }
        return None;
    }
}


pub enum HibouPreFilter {
    MaxLoopInstanciation(u32),
    MaxProcessDepth(usize),
    MaxNodeNumber(u32)
}

impl std::string::ToString for HibouPreFilter {
    fn to_string(&self) -> String {
        match self {
            HibouPreFilter::MaxLoopInstanciation(num) => {
                return format!("MaxLoop={}",num);
            },
            HibouPreFilter::MaxProcessDepth(num) => {
                return format!("MaxDepth={}",num);
            },
            HibouPreFilter::MaxNodeNumber(num) => {
                return format!("MaxNum={}",num);
            }
        }
    }
}

pub enum FilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl std::string::ToString for FilterEliminationKind {
    fn to_string(&self) -> String {
        match self {
            FilterEliminationKind::MaxLoopInstanciation => {
                return "MaxLoop".to_string();
            },
            FilterEliminationKind::MaxProcessDepth => {
                return "MaxDepth".to_string();
            },
            FilterEliminationKind::MaxNodeNumber => {
                return "MaxNum".to_string();
            }
        }
    }
}


pub enum HibouSearchStrategy {
    BFS,
    DFS
}

impl std::string::ToString for HibouSearchStrategy {
    fn to_string(&self) -> String {
        match self {
            HibouSearchStrategy::BFS => {
                return "Breadth First Search".to_string();
            },
            HibouSearchStrategy::DFS => {
                return "Depth First Search".to_string();
            }
        }
    }
}

pub enum SemanticKind {
    Accept,
    Prefix
}


impl std::string::ToString for SemanticKind {
    fn to_string(&self) -> String {
        match self {
            SemanticKind::Accept => {
                return "accept".to_string();
            },
            SemanticKind::Prefix => {
                return "prefix".to_string();
            }
        }
    }
}