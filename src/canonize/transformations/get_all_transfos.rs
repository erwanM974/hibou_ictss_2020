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

use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::rendering::textual::monochrome::position::position_to_text;

use crate::canonize::transformations::transfokind::*;
use crate::canonize::transformations::transfodef::*;

use crate::canonize::transformations::phases::*;


pub fn phase_1_all_transfos(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    return get_all_transformations_rec(&transfos_phase1(),interaction);
}

pub fn phase_2_all_transfos(interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    return get_all_transformations_rec(&transfos_phase2(),interaction);
}

fn get_all_transformations_inner(transfos : &Vec<(TransformationKind, &dyn Fn(&Interaction) -> Option<Interaction>)>,
                                 interaction : &Interaction) -> Vec<InteractionTermTransformation> {

    let mut results : Vec<InteractionTermTransformation> = Vec::new();
    for (transfo_kind, transfo_func) in transfos {
        match transfo_func(interaction) {
            None => {},
            Some(new_int) => {
                results.push( InteractionTermTransformation::new((*transfo_kind).clone(),Position::Epsilon,new_int) );
            }
        }
    }
    return results;
}

fn get_all_transformations_rec(transfos : &Vec<(TransformationKind, &dyn Fn(&Interaction) -> Option<Interaction>)>,
                               interaction : &Interaction) -> Vec<InteractionTermTransformation> {
    let mut results = get_all_transformations_inner(transfos,interaction);
    match interaction {
        &Interaction::Empty => {
            // ***
        }, &Interaction::Action(_) => {
            // ***
        }, &Interaction::Strict(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Strict(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Strict(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Seq(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Seq(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Seq(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::CoReg(ref cr, ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::CoReg(cr.clone(), Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::CoReg(cr.clone(), i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Par(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Par(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Par(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Alt(ref i1, ref i2) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Alt(Box::new(left_transfo.result),i2.clone())
                ) );
            }
            for right_transfo in get_all_transformations_rec(transfos,i2) {
                results.push( InteractionTermTransformation::new(right_transfo.kind,
                                                                         Position::Right(Box::new(right_transfo.position)),
                                                                         Interaction::Alt(i1.clone(), Box::new(right_transfo.result))
                ) );
            }
        }, &Interaction::Loop(ref lk, ref i1) => {
            for left_transfo in get_all_transformations_rec(transfos,i1) {
                results.push( InteractionTermTransformation::new(left_transfo.kind,
                                                                         Position::Left(Box::new(left_transfo.position)),
                                                                         Interaction::Loop(lk.clone(), Box::new(left_transfo.result))
                ) );
            }
        }
    }
    return results;
}


