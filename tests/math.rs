use rhai::{Engine, EvalAltResult, INT};

#[test]
fn test_math() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    assert_eq!(engine.eval::<INT>("1 + 2")?, 3);
    assert_eq!(engine.eval::<INT>("1 - 2")?, -1);
    assert_eq!(engine.eval::<INT>("2 * 3")?, 6);
    assert_eq!(engine.eval::<INT>("1 / 2")?, 0);
    assert_eq!(engine.eval::<INT>("3 % 2")?, 1);

    #[cfg(not(feature = "only_i32"))]
    assert_eq!(
        engine.eval::<INT>("abs(-9223372036854775807)")?,
        9_223_372_036_854_775_807
    );

    #[cfg(feature = "only_i32")]
    assert_eq!(engine.eval::<INT>("abs(-2147483647)")?, 2147483647);

    // Overflow/underflow/division-by-zero errors
    #[cfg(not(feature = "unchecked"))]
    {
        #[cfg(not(feature = "only_i32"))]
        {
            assert!(matches!(
                engine
                    .eval::<INT>("abs(-9223372036854775808)")
                    .expect_err("expects negation overflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("9223372036854775807 + 1")
                    .expect_err("expects overflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("-9223372036854775808 - 1")
                    .expect_err("expects underflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("9223372036854775807 * 9223372036854775807")
                    .expect_err("expects overflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("9223372036854775807 / 0")
                    .expect_err("expects division by zero"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("9223372036854775807 % 0")
                    .expect_err("expects division by zero"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
        }

        #[cfg(feature = "only_i32")]
        {
            assert!(matches!(
                engine
                    .eval::<INT>("2147483647 + 1")
                    .expect_err("expects overflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("-2147483648 - 1")
                    .expect_err("expects underflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("2147483647 * 2147483647")
                    .expect_err("expects overflow"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("2147483647 / 0")
                    .expect_err("expects division by zero"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
            assert!(matches!(
                engine
                    .eval::<INT>("2147483647 % 0")
                    .expect_err("expects division by zero"),
                EvalAltResult::ErrorArithmetic(_, _)
            ));
        }
    }

    Ok(())
}
