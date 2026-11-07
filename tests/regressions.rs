use rust_ds_lab::{extras, model, progress::Progress};
#[test]
fn direct_build_rejects_malformed_pairs_without_panicking() {
    for kind in ["hashmap", "graph", "dsu"] {
        for value in ["missing", ":", "-", "a:", "-b", "a:b:c", "a-b-c", "é:3"] {
            let items = vec![value.to_owned()];
            if model::validate(kind, &items).is_err() {
                assert!(model::try_build(kind, &items).is_err());
                let d = model::build(kind, &items);
                assert!(d.nodes.is_empty());
                assert_eq!(d.rows[0].0, "Input error");
            }
        }
    }
    assert!(model::try_build("unknown", &[]).is_err());
    assert!(model::try_build("array", &vec!["1".into(); 17]).is_err());
    assert!(model::try_build("array", &["99999999999999999999999".into()]).is_err());
}
#[test]
fn legacy_checkpoints_survive_and_new_topics_start_unchecked() {
    let p = Progress::decode(&format!(
        "version=1\ncheckpoints={}\nrecent=33\n",
        "7".repeat(34)
    ));
    for i in 0..34 {
        assert!(p.completed(i));
    }
    for i in 34..47 {
        assert!(!p.completed(i));
    }
    assert_eq!(p.recent, Some(33));
    assert_eq!(Progress::decode(&p.encode()), p);
}
#[test]
fn all_new_cases_produce_bounded_finite_connected_diagrams() {
    for topic in 0..extras::TOPICS.len() {
        for case in 0..10 {
            let e = extras::trace(topic, &extras::example(topic, case), case).unwrap();
            assert!(!e.frames.is_empty() && e.frames.len() < 1500);
            assert!(!e.answer.is_empty());
            for f in e.frames {
                if let Some(d) = f.diagram {
                    assert!(d.width.is_finite() && d.height.is_finite());
                    assert!(d.nodes.len() <= 64);
                    assert!(d.nodes.iter().all(|n| n.x.is_finite() && n.y.is_finite()));
                    assert!(d
                        .edges
                        .iter()
                        .all(|e| e.from < d.nodes.len() && e.to < d.nodes.len()));
                }
            }
        }
    }
}
