use rust_ds_lab::{
    curriculum::{LESSONS, LINKS},
    practice,
};
#[test]
fn roadmap_is_complete_and_acyclic() {
    assert_eq!(LESSONS.len(), 18);
    assert_eq!(rust_ds_lab::guides::GUIDES.len(), LESSONS.len());
    for &(a, b) in LINKS {
        assert!(a < LESSONS.len() && b < LESSONS.len());
        assert!(LESSONS[a].y < LESSONS[b].y);
    }
}
#[test]
fn every_exercise_produces_bounded_explained_frames() {
    assert_eq!(practice::total_questions(), 180);
    for t in 0..18 {
        for q in 0..10 {
            let e = practice::exercise(t, q);
            assert!(
                !e.prompt.is_empty()
                    && !e.answer.is_empty()
                    && !e.explanation.is_empty()
                    && !e.cost.is_empty()
            );
            assert!(!e.frames.is_empty() && e.frames.len() <= 128, "{t}:{q}");
            for f in &e.frames {
                assert!(f.columns > 0);
                for &i in &f.active {
                    assert!(i < f.cells.len(), "{t}:{q}");
                }
                for &(a, b) in &f.edges {
                    assert!(a < f.cells.len() && b < f.cells.len());
                }
            }
        }
    }
}
#[test]
fn known_solutions_match_independent_values() {
    assert_eq!(
        practice::exercise(10, 0).answer,
        "Distances from 0: [0, 1, 2, 2, 1]."
    );
    assert_eq!(
        practice::exercise(14, 0).answer,
        "Distances from 0: [0, 1, 3, 6, 2]."
    );
    assert!(practice::exercise(11, 0).answer.starts_with("3 "));
    assert!(practice::exercise(15, 0).answer.starts_with("2 "));
    assert!(practice::exercise(16, 0).answer.starts_with("3 "));
    assert!(practice::exercise(17, 0).answer.contains("6."));
    assert!(practice::exercise(2, 2).answer.contains("false"));
    assert!(practice::exercise(9, 0).answer.starts_with("8 subsets"));
}
#[test]
fn every_structure_has_ten_valid_operation_traces() {
    for t in 0..16 {
        for c in 0..10 {
            let e = practice::structure_exercise(t, c);
            assert!(!e.answer.is_empty());
            assert!(e.frames.len() >= 2);
            for f in &e.frames {
                assert!(!f.note.contains("rejected"), "{t}:{c}: {}", f.note);
            }
        }
    }
}
