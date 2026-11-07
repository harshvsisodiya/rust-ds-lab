use super::*;

fn draw(ctx: &egui::Context, lab: &mut Lab, size: [f32; 2]) -> egui::FullOutput {
    ctx.run(
        egui::RawInput {
            time: Some(ctx.cumulative_pass_nr() as f64 * 0.5),

            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::from(size))),

            ..Default::default()
        },
        |ctx| lab.show(ctx),
    )
}

#[test]
fn all_native_views_render_at_small_and_large_sizes() {
    let ctx = egui::Context::default();

    let mut lab = Lab::with_context(&ctx);

    lab.storage.progress.reduced_motion = true;

    for size in [[800.0, 600.0], [1280.0, 860.0]] {
        let check = |lab: &mut Lab| {
            let o = draw(&ctx, lab, size);

            assert!(!ctx.tessellate(o.shapes, o.pixels_per_point).is_empty());
        };

        for v in [View::Home, View::Library, View::Roadmap] {
            lab.nav.view = v;

            check(&mut lab);
        }

        for i in 0..LESSONS.len() {
            lab.select_lesson(i);

            check(&mut lab);

            lab.nav.tab = Tab::Playground;

            check(&mut lab);

            lab.start_practice(false);

            lab.study.reveal = true;

            check(&mut lab);

            lab.playback.step = lab.study.exercise.frames.len() - 1;

            check(&mut lab);
        }

        for i in 0..TOPICS.len() {
            lab.select_structure(i);

            for tab in [Tab::Understand, Tab::Playground, Tab::Code] {
                lab.nav.tab = tab;

                check(&mut lab);
            }

            lab.start_practice(true);

            lab.study.case = 9;

            lab.load_exercise();

            lab.study.reveal = true;

            check(&mut lab);

            lab.playback.step = lab.study.exercise.frames.len() - 1;

            check(&mut lab);
        }

        for i in 0..crate::extras::TOPICS.len() {
            lab.select_extra(i);
            for tab in [Tab::Understand, Tab::Playground, Tab::Code] {
                lab.nav.tab = tab;
                check(&mut lab);
            }
            lab.playback.step = lab.study.exercise.frames.len() - 1;
            check(&mut lab);
        }
        for i in 0..4 {
            lab.nav.tutorial = Some(i);

            check(&mut lab);
        }

        lab.nav.tutorial = None;
    }
}

#[test]
fn navigation_stops_playback_and_example_inputs_are_valid() {
    let ctx = egui::Context::default();

    let mut lab = Lab::with_context(&ctx);

    for (i, t) in TOPICS.iter().enumerate() {
        lab.playback.play = true;

        lab.select_structure(i);

        assert!(!lab.playback.play);

        assert_eq!(lab.nav.tab, Tab::Understand);

        assert!(model::validate(t.id, &lab.editor.items).is_ok());

        lab.action("insert");

        assert!(
            !lab.editor.message.contains("must"),
            "{}: {}",
            t.id,
            lab.editor.message
        );
    }

    lab.playback.play = true;

    lab.select_lesson(3);

    assert_eq!(lab.playback.step, 0);

    assert!(!lab.playback.play);
}

#[test]
fn real_buttons_navigate_and_idle_views_stop_repainting() {
    let ctx = egui::Context::default();

    let mut lab = Lab::with_context(&ctx);

    lab.storage.progress.reduced_motion = true;

    let click = |lab: &mut Lab, text: &str| {
        let mut output = draw(&ctx, lab, [1280.0, 900.0]);

        for _ in 0..3 {
            output = draw(&ctx, lab, [1280.0, 900.0]);
        }

        let pos = output
            .shapes
            .iter()
            .find_map(|s| {
                if let egui::Shape::Text(t) = &s.shape {
                    if t.galley.job.text == text {
                        return Some(t.pos + t.galley.rect.center().to_vec2());
                    }
                }

                None
            })
            .unwrap_or_else(|| panic!("missing visible control: {text}"));

        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 900.0))),

                time: Some(ctx.cumulative_pass_nr() as f64 * 0.5),

                events: vec![
                    egui::Event::PointerMoved(pos),
                    egui::Event::PointerButton {
                        pos,

                        button: egui::PointerButton::Primary,

                        pressed: true,

                        modifiers: egui::Modifiers::NONE,
                    },
                    egui::Event::PointerButton {
                        pos,

                        button: egui::PointerButton::Primary,

                        pressed: false,

                        modifiers: egui::Modifiers::NONE,
                    },
                ],

                ..Default::default()
            },
            |ctx| lab.show(ctx),
        );
    };

    click(&mut lab, "Continue learning  →");

    assert_eq!(lab.nav.view, View::Lesson);

    click(&mut lab, "02  Visual walkthrough");

    assert_eq!(lab.nav.tab, Tab::Playground);

    click(&mut lab, "Next →");

    assert_eq!(lab.playback.step, 1);

    click(&mut lab, "Start here");

    assert_eq!(lab.nav.view, View::Home);

    let mut output = draw(&ctx, &mut lab, [1280.0, 900.0]);

    for _ in 0..10 {
        output = draw(&ctx, &mut lab, [1280.0, 900.0]);
    }

    assert!(
        output.viewport_output[&egui::ViewportId::ROOT].repaint_delay > Duration::from_millis(100)
    );
}

/// Exports real egui meshes and font atlas for offline visual review.
#[test]
#[ignore]
fn export_native_previews() {
    let dir = std::env::var("DS_LAB_PREVIEW").expect("set DS_LAB_PREVIEW");

    std::fs::create_dir_all(&dir).unwrap();

    for name in [
        "home",
        "lesson",
        "playground",
        "practice",
        "library",
        "roadmap",
        "tour",
        "small",
        "completed",
        "small_lab",
        "small_tour",
        "sorting",
        "avl",
        "redblack",
        "btree",
        "search",
        "extra_library",
        "small_sorting",
    ] {
        let ctx = egui::Context::default();

        let mut lab = Lab::with_context(&ctx);

        lab.storage.progress.reduced_motion = true;

        match name {
            "sorting" | "small_sorting" => {
                lab.select_extra(3);
                lab.nav.tab = Tab::Playground;
                lab.playback.step = 4;
            }
            "avl" => {
                lab.select_extra(6);
                lab.nav.tab = Tab::Playground;
                lab.playback.step = lab.study.exercise.frames.len() - 1;
            }
            "redblack" => {
                lab.select_extra(7);
                lab.nav.tab = Tab::Playground;
                lab.playback.step = lab.study.exercise.frames.len() - 1;
            }
            "btree" => {
                lab.select_extra(8);
                lab.nav.tab = Tab::Playground;
                lab.playback.step = lab.study.exercise.frames.len() - 1;
            }
            "search" => {
                lab.nav.command_open = true;
                lab.nav.command_query = "tree".into();
            }
            "extra_library" => {
                lab.nav.view = View::Library;
                lab.nav.library_group = 2;
            }
            "completed" => {
                for stage in 0..3 {
                    lab.storage.progress.set(0, stage, true);
                }

                lab.storage.progress.set(1, 0, true);
            }

            "small_lab" => {
                lab.select_structure(9);

                lab.nav.tab = Tab::Playground;
            }

            "small_tour" => lab.nav.tutorial = Some(3),

            "lesson" => lab.select_lesson(3),

            "playground" => {
                lab.select_structure(9);

                lab.nav.tab = Tab::Playground;
            }

            "practice" => {
                lab.select_lesson(3);

                lab.start_practice(false);

                lab.study.reveal = true;

                lab.playback.step = 1;
            }

            "library" => lab.nav.view = View::Library,

            "roadmap" => lab.nav.view = View::Roadmap,

            "tour" => lab.nav.tutorial = Some(0),

            _ => (),
        }

        let size = if name.starts_with("small") {
            [800.0, 600.0]
        } else {
            [1280.0, 900.0]
        };

        let mut textures = Vec::new();

        let mut meshes = Vec::new();

        for _ in 0..3 {
            let output = draw(&ctx, &mut lab, size);

            for (id, delta) in output.textures_delta.set {
                let (w, h) = (delta.image.width(), delta.image.height());

                let pixels: Vec<[u8; 4]> = match &delta.image {
                    egui::ImageData::Color(im) => im.pixels.iter().map(|c| c.to_array()).collect(),

                    egui::ImageData::Font(im) => {
                        im.srgba_pixels(None).map(|c| c.to_array()).collect()
                    }
                };

                textures.push(serde_json::json!({
                "id":format!("{id:?}"),
                "w":w,
                "h":h,
                "pos":delta.pos,
                "pixels":pixels}
                ));
            }

            meshes = ctx
                .tessellate(output.shapes, output.pixels_per_point)
                .into_iter()
                .filter_map(|p| {
                    if let egui::epaint::Primitive::Mesh(m) = p.primitive {
                        Some(serde_json::json!({
                        "texture":format!("{:?}",
                        m.texture_id),
                        "clip":[p.clip_rect.min.x,
                        p.clip_rect.min.y,
                        p.clip_rect.max.x,
                        p.clip_rect.max.y],
                        "indices":m.indices,
                        "vertices":m.vertices.iter().map(|v|[v.pos.x,
                        v.pos.y,
                        v.uv.x,
                        v.uv.y,
                        v.color.r() as f32,
                        v.color.g() as f32,
                        v.color.b() as f32,
                        v.color.a() as f32]).collect::<Vec<_>>()}
                        ))
                    } else {
                        None
                    }
                })
                .collect();
        }

        std::fs::write(
            format!("{dir}/{name}.json"),
            serde_json::to_vec(&serde_json::json!({
            "size":size,
            "textures":textures,
            "meshes":meshes}
            ))
            .unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn session_round_trip_restores_custom_input_case_and_paused_playback() {
    let ctx = egui::Context::default();
    let mut lab = Lab::with_context(&ctx);
    lab.select_extra(3);
    lab.study.case = 8;
    lab.editor.data = "9,2,7,1".into();
    assert!(lab.build_extra());
    lab.nav.tab = Tab::Playground;
    lab.playback.step = 3;
    lab.playback.play = true;
    lab.playback.speed = 2.5;
    lab.study.answer = "Sorted: 1,2,7,9".into();
    lab.nav.scroll = 123.0;
    lab.nav.text_scale = 1.1;
    let session = lab.session_text();
    let mut restored = Lab::with_context(&ctx);
    assert!(restored.restore_session(&session));
    assert_eq!(restored.route(), lab.route());
    assert_eq!(restored.editor.data, lab.editor.data);
    assert_eq!(restored.study.answer, lab.study.answer);
    assert_eq!(restored.playback.step, 3);
    assert!(!restored.playback.play);
    assert_eq!(restored.playback.speed, 2.5);
    assert_eq!(restored.nav.scroll, 123.0);
    assert_eq!(restored.nav.text_scale, 1.1);
    assert_eq!(restored.study.exercise.answer, lab.study.exercise.answer);
    lab.select_structure(7);
    lab.editor.data = "cat:7,dog:2".into();
    lab.editor.items = model::tokens(&lab.editor.data);
    lab.playback.step = 1;
    lab.rebuild();
    assert!(restored.restore_session(&lab.session_text()));
    assert_eq!(restored.editor.items, lab.editor.items);
    assert_eq!(restored.playback.step, 1);
}
#[test]
fn malformed_sessions_do_not_mutate_the_current_page() {
    let ctx = egui::Context::default();
    let mut lab = Lab::with_context(&ctx);
    lab.select_extra(8);
    let before = lab.session_text();
    for invalid in ["", "ff", "not-hex", &"a".repeat(16002)] {
        assert!(!lab.restore_session(invalid));
        assert_eq!(lab.session_text(), before);
    }
    let mut bad = Lab::with_context(&ctx);
    bad.select_structure(7);
    bad.editor.data = "missing-colon".into();
    assert!(!lab.restore_session(&bad.session_text()));
    assert_eq!(lab.session_text(), before);
    bad.select_extra(12);
    bad.editor.data = "missing separator".into();
    assert!(!lab.restore_session(&bad.session_text()));
    assert_eq!(lab.session_text(), before);
}
#[test]
fn history_restores_exercise_and_new_navigation_discards_forward_stack() {
    let ctx = egui::Context::default();
    let mut lab = Lab::with_context(&ctx);
    lab.remember_route();
    lab.select_extra(6);
    lab.study.case = 7;
    lab.editor.data = crate::extras::example(6, 7);
    lab.build_extra();
    lab.remember_route();
    let answer = lab.study.exercise.answer.clone();
    lab.select_lesson(3);
    lab.remember_route();
    lab.history_back();
    lab.remember_route();
    assert_eq!(lab.nav.view, View::Extensions);
    assert_eq!(lab.study.case, 7);
    assert_eq!(lab.study.exercise.answer, answer);
    lab.history_forward();
    lab.remember_route();
    assert_eq!(lab.nav.lesson, 3);
    lab.history_back();
    lab.remember_route();
    lab.select_structure(2);
    lab.remember_route();
    assert!(lab.nav.forward.is_empty());
}
#[test]
fn shortcuts_step_pause_search_and_respect_text_input() {
    let ctx = egui::Context::default();
    let mut lab = Lab::with_context(&ctx);
    lab.select_extra(3);
    lab.nav.tab = Tab::Playground;
    let key = |lab: &mut Lab, key, modifiers| {
        let _ = ctx.run(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1280.0, 900.0))),
                modifiers,
                events: vec![egui::Event::Key {
                    key,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers,
                }],
                ..Default::default()
            },
            |ctx| lab.show(ctx),
        );
    };
    draw(&ctx, &mut lab, [1280.0, 900.0]);
    key(&mut lab, egui::Key::ArrowRight, egui::Modifiers::NONE);
    assert_eq!(lab.playback.step, 1);
    key(&mut lab, egui::Key::Space, egui::Modifiers::NONE);
    assert!(lab.playback.play);
    key(&mut lab, egui::Key::K, egui::Modifiers::COMMAND);
    assert!(lab.nav.command_open);
    assert!(!lab.playback.play);
    key(&mut lab, egui::Key::ArrowRight, egui::Modifiers::NONE);
    assert_eq!(lab.playback.step, 1);
    key(&mut lab, egui::Key::Escape, egui::Modifiers::NONE);
    assert!(!lab.nav.command_open);
    // A focused text field owns arrows and Space, preventing accidental replay.
    ctx.memory_mut(|m| m.request_focus(egui::Id::new("typing")));
    key(&mut lab, egui::Key::ArrowRight, egui::Modifiers::NONE);
    assert_eq!(lab.playback.step, 1);
}
