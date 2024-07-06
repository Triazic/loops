// we construct a nested structure of rails

use crate::{rail::Rail, rail_edge::rail_edge, solver_types::{Direction, SolverState}, xy::xy};

// /** a square with just one rail */
// pub fn test_case_square_1() -> Rail {
//     let outer_rail = Rail {
//         id: 0,
//         edges: Vec::from([
//             rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
//             rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
//             rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
//             rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
//         ]),
//         child_rails: Vec::new(),
//     };
//     outer_rail
// }

// /** a square with one inner rail and an outer rail */
// pub fn test_case_square_2() -> Rail {
//     let outer_rail = Rail {
//         id: 0,
//         edges: Vec::from([
//             rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
//             rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
//             rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
//             rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
//         ]),
//         child_rails: Vec::from([
//             Rail {
//                 id: 1,
//                 edges: Vec::from([
//                     rail_edge(xy(0.2, 0.2), xy(0.2, 0.8), 4, Some(0)), // LEFT
//                     rail_edge(xy(0.2, 0.8), xy(0.8, 0.8), 5, Some(1)), // TOP
//                     rail_edge(xy(0.8, 0.8), xy(0.8, 0.2), 6, Some(2)), // RIGHT
//                     rail_edge(xy(0.8, 0.2), xy(0.2, 0.2), 7, Some(3)), // BOTTOM
//                 ]),
//                 child_rails: Vec::new()
//             }
//         ])
//     };
//     outer_rail
// }

// /** a square with an outer rail and two inner rails */
// pub fn test_case_square_3() -> Rail {
//     let outer_rail = Rail {
//         id: 0,
//         edges: Vec::from([
//             rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
//             rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
//             rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
//             rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
//         ]),
//         child_rails: Vec::from([
//             Rail {
//                 id: 1,
//                 edges: Vec::from([
//                     rail_edge(xy(0.2, 0.2), xy(0.2, 0.8), 4, Some(0)), // LEFT
//                     rail_edge(xy(0.2, 0.8), xy(0.8, 0.8), 5, Some(1)), // TOP
//                     rail_edge(xy(0.8, 0.8), xy(0.8, 0.2), 6, Some(2)), // RIGHT
//                     rail_edge(xy(0.8, 0.2), xy(0.2, 0.2), 7, Some(3)), // BOTTOM
//                 ]),
//                 child_rails: Vec::from([
//                     Rail {
//                         id: 2,
//                         edges: Vec::from([
//                             rail_edge(xy(0.4, 0.4), xy(0.4, 0.6), 8,  Some(4)), // LEFT
//                             rail_edge(xy(0.4, 0.6), xy(0.6, 0.6), 9,  Some(5)), // TOP
//                             rail_edge(xy(0.6, 0.6), xy(0.6, 0.4), 10, Some(6)), // RIGHT
//                             rail_edge(xy(0.6, 0.4), xy(0.4, 0.4), 11, Some(7)), // BOTTOM
//                         ]),
//                         child_rails: Vec::new()
//                     }
//                 ])
//             }
//         ])
//     };
//     outer_rail
// }

// /** a square with an outer rail and three inner rails */
// pub fn test_case_square_4() -> Rail {
//     let outer_rail = Rail {
//         id: 0,
//         edges: Vec::from([
//             rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
//             rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
//             rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
//             rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
//         ]),
//         child_rails: Vec::from([
//             Rail {
//                 id: 1,
//                 edges: Vec::from([
//                     rail_edge(xy(0.05, 0.05), xy(0.05, 0.95),  4, Some(0)), // LEFT
//                     rail_edge(xy(0.05, 0.95), xy(0.95, 0.95), 5, Some(1)), // TOP
//                     rail_edge(xy(0.95, 0.95), xy(0.95, 0.05), 6, Some(2)), // RIGHT
//                     rail_edge(xy(0.95, 0.05), xy(0.05, 0.05), 7, Some(3)), // BOTTOM
//                 ]),
//                 child_rails: Vec::from([
//                     Rail {
//                         id: 2,
//                         edges: Vec::from([
//                             rail_edge(xy(0.1, 0.1), xy(0.1, 0.9), 8,  Some(4)), // LEFT
//                             rail_edge(xy(0.1, 0.9), xy(0.9, 0.9), 9,  Some(5)), // TOP
//                             rail_edge(xy(0.9, 0.9), xy(0.9, 0.1), 10, Some(6)), // RIGHT
//                             rail_edge(xy(0.9, 0.1), xy(0.1, 0.1), 11, Some(7)), // BOTTOM
//                         ]),
//                         child_rails: Vec::from([
//                             Rail {
//                                 id: 3,
//                                 edges: Vec::from([
//                                     rail_edge(xy(0.15, 0.15), xy(0.15, 0.85), 12,  Some(8)), // LEFT
//                                     rail_edge(xy(0.15, 0.85), xy(0.85, 0.85), 13,  Some(9)), // TOP
//                                     rail_edge(xy(0.85, 0.85), xy(0.85, 0.15), 14,  Some(10)), // RIGHT
//                                     rail_edge(xy(0.85, 0.15), xy(0.15, 0.15), 15,  Some(11)), // BOTTOM
//                                 ]),
//                                 child_rails: Vec::new()
//                             }
//                         ])
//                     }
//                 ])
//             }
//         ])
//     };
//     outer_rail
// }

// /** a square with an outer rail and four inner rails */
// pub fn test_case_square_5() -> Rail {
//     let outer_rail = Rail {
//         id: 0,
//         edges: Vec::from([
//             rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
//             rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
//             rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
//             rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
//         ]),
//         child_rails: Vec::from([
//             Rail {
//                 id: 1,
//                 edges: Vec::from([
//                     rail_edge(xy(0.05, 0.05), xy(0.05, 0.95),  4, Some(0)), // LEFT
//                     rail_edge(xy(0.05, 0.95), xy(0.95, 0.95), 5, Some(1)), // TOP
//                     rail_edge(xy(0.95, 0.95), xy(0.95, 0.05), 6, Some(2)), // RIGHT
//                     rail_edge(xy(0.95, 0.05), xy(0.05, 0.05), 7, Some(3)), // BOTTOM
//                 ]),
//                 child_rails: Vec::from([
//                     Rail {
//                         id: 2,
//                         edges: Vec::from([
//                             rail_edge(xy(0.1, 0.1), xy(0.1, 0.9), 8,  Some(4)), // LEFT
//                             rail_edge(xy(0.1, 0.9), xy(0.9, 0.9), 9,  Some(5)), // TOP
//                             rail_edge(xy(0.9, 0.9), xy(0.9, 0.1), 10, Some(6)), // RIGHT
//                             rail_edge(xy(0.9, 0.1), xy(0.1, 0.1), 11, Some(7)), // BOTTOM
//                         ]),
//                         child_rails: Vec::from([
//                             Rail {
//                                 id: 3,
//                                 edges: Vec::from([
//                                     rail_edge(xy(0.15, 0.15), xy(0.15, 0.85), 12,  Some(8)), // LEFT
//                                     rail_edge(xy(0.15, 0.85), xy(0.85, 0.85), 13,  Some(9)), // TOP
//                                     rail_edge(xy(0.85, 0.85), xy(0.85, 0.15), 14,  Some(10)), // RIGHT
//                                     rail_edge(xy(0.85, 0.15), xy(0.15, 0.15), 15,  Some(11)), // BOTTOM
//                                 ]),
//                                 child_rails: Vec::from([
//                                     Rail {
//                                         id: 4,
//                                         edges: Vec::from([
//                                             rail_edge(xy(0.2, 0.2), xy(0.2, 0.8), 16,  Some(12)), // LEFT
//                                             rail_edge(xy(0.2, 0.8), xy(0.8, 0.8), 17,  Some(13)), // TOP
//                                             rail_edge(xy(0.8, 0.8), xy(0.8, 0.2), 18,  Some(14)), // RIGHT
//                                             rail_edge(xy(0.8, 0.2), xy(0.2, 0.2), 19,  Some(15)), // BOTTOM
//                                         ]),
//                                         child_rails: Vec::new()
//                                     }
//                                 ])
//                             }
//                         ])
//                     }
//                 ])
//             }
//         ])
//     };
//     outer_rail
// }

/** a square with an outer rail and five inner rails */
pub fn test_case_square_6() -> SolverState {
    let outer_rail = Rail {
        id: 0,
        parent_rail_id: None,
        edges: Vec::from([
            rail_edge(xy(0., 0.), xy(0., 1.), 0, None), // LEFT
            rail_edge(xy(0., 1.), xy(1., 1.), 1, None), // TOP
            rail_edge(xy(1., 1.), xy(1., 0.), 2, None), // RIGHT
            rail_edge(xy(1., 0.), xy(0., 0.), 3, None), // BOTTOM
        ]),
        child_rails: Vec::from([
            Rail {
                id: 1,
                parent_rail_id: Some(0),
                edges: Vec::from([
                    rail_edge(xy(0.05, 0.05), xy(0.05, 0.95),  4, Some(0)), // LEFT
                    rail_edge(xy(0.05, 0.95), xy(0.95, 0.95), 5, Some(1)), // TOP
                    rail_edge(xy(0.95, 0.95), xy(0.95, 0.05), 6, Some(2)), // RIGHT
                    rail_edge(xy(0.95, 0.05), xy(0.05, 0.05), 7, Some(3)), // BOTTOM
                ]),
                child_rails: Vec::from([
                    Rail {
                        id: 2,
                        parent_rail_id: Some(1),
                        edges: Vec::from([
                            rail_edge(xy(0.1, 0.1), xy(0.1, 0.9), 8,  Some(4)), // LEFT
                            rail_edge(xy(0.1, 0.9), xy(0.9, 0.9), 9,  Some(5)), // TOP
                            rail_edge(xy(0.9, 0.9), xy(0.9, 0.1), 10, Some(6)), // RIGHT
                            rail_edge(xy(0.9, 0.1), xy(0.1, 0.1), 11, Some(7)), // BOTTOM
                        ]),
                        child_rails: Vec::from([
                            Rail {
                                id: 3,
                                parent_rail_id: Some(2),
                                edges: Vec::from([
                                    rail_edge(xy(0.15, 0.15), xy(0.15, 0.85), 12,  Some(8)), // LEFT
                                    rail_edge(xy(0.15, 0.85), xy(0.85, 0.85), 13,  Some(9)), // TOP
                                    rail_edge(xy(0.85, 0.85), xy(0.85, 0.15), 14,  Some(10)), // RIGHT
                                    rail_edge(xy(0.85, 0.15), xy(0.15, 0.15), 15,  Some(11)), // BOTTOM
                                ]),
                                child_rails: Vec::from([
                                    Rail {
                                        id: 4,
                                        parent_rail_id: Some(3),
                                        edges: Vec::from([
                                            rail_edge(xy(0.2, 0.2), xy(0.2, 0.8), 16,  Some(12)), // LEFT
                                            rail_edge(xy(0.2, 0.8), xy(0.8, 0.8), 17,  Some(13)), // TOP
                                            rail_edge(xy(0.8, 0.8), xy(0.8, 0.2), 18,  Some(14)), // RIGHT
                                            rail_edge(xy(0.8, 0.2), xy(0.2, 0.2), 19,  Some(15)), // BOTTOM
                                        ]),
                                        child_rails: Vec::from([
                                            Rail {
                                                id: 5,
                                                parent_rail_id: Some(4),
                                                edges: Vec::from([
                                                    rail_edge(xy(0.25, 0.25), xy(0.25, 0.75), 20,  Some(16)), // LEFT
                                                    rail_edge(xy(0.25, 0.75), xy(0.75, 0.75), 21,  Some(17)), // TOP
                                                    rail_edge(xy(0.75, 0.75), xy(0.75, 0.25), 22,  Some(18)), // RIGHT
                                                    rail_edge(xy(0.75, 0.25), xy(0.25, 0.25), 23,  Some(19)), // BOTTOM
                                                ]),
                                                child_rails: Vec::new()
                                            }
                                        ])
                                    }
                                ])
                            }
                        ])
                    }
                ])
            }
        ])
    };
    SolverState {
        root_rail: outer_rail,
        seed_point: xy(0.1, -0.1),
        seed_direction: Direction::Clockwise,
        pipe_spacing: 0.05,
    }
}