pub mod block_generator_into {
    use sb_itchy::prelude::*;
    use sb_sbity::value::{Number, Value};
    pub type Bfb = BlockFieldBuilder;
    pub type Bib = BlockInputBuilder;
    pub type Biv = BlockInputValue;

    pub trait BlockGeneratorInto<T> {
        fn to(self) -> T;
    }

    impl BlockGeneratorInto<Bib> for i32 {
        #[inline]
        fn to(self) -> Bib {
            Bib::value(Biv::Number {
                value: Value::Number(Number::Int(self as i64)),
            })
        }
    }

    impl BlockGeneratorInto<Bib> for usize {
        #[inline]
        fn to(self) -> Bib {
            Bib::value(Biv::Number {
                value: Value::Number(Number::Int(self as i64)),
            })
        }
    }

    impl BlockGeneratorInto<Bib> for &str {
        #[inline]
        fn to(self) -> Bib {
            Bib::value(Biv::String {
                value: Value::Text(self.into()),
            })
        }
    }

    impl BlockGeneratorInto<Bib> for bool {
        #[inline]
        fn to(self) -> Bib {
            Bib::value(Biv::String {
                value: Value::Text(self.to_string().into()),
            })
        }
    }

    impl BlockGeneratorInto<Bfb> for &str {
        #[inline]
        fn to(self) -> Bfb {
            Bfb::new(self.into())
        }
    }

    impl BlockGeneratorInto<Option<Bib>> for StackBuilder {
        #[inline]
        fn to(self) -> Option<Bib> {
            Some(Bib::stack(self))
        }
    }

    impl<U: Into<String>> BlockGeneratorInto<String> for U {
        #[inline]
        fn to(self) -> String {
            self.into()
        }
    }

    macro_rules! to_self {
        ($($typ:ty),*) => {
            $(impl BlockGeneratorInto<$typ> for $typ {
                #[inline]
                fn to(self) -> $typ {
                    self
                }
            })*
        };
    }

    to_self!(Bib, Bfb, usize, i32, bool);

    macro_rules! ref_to {
        ($($typ:ty),*) => {
            $(impl<T: BlockGeneratorInto<$typ> + Clone> BlockGeneratorInto<$typ> for &T {
                #[inline]
                fn to(self) -> $typ {
                    let tmp: T = self.clone();
                    tmp.to()
                }
            })*
        };
    }

    ref_to!(Bib, Bfb, usize, i32, bool);

    #[macro_export]
    macro_rules! stack {
        () => (
            (sb_itchy::stack::StackBuilder::new())
        );
        // ($($x:expr),+ $(,)?) => (
        //     (sb_itchy::stack::StackBuilder::new().next($($x),+))
        // );
        ($($x:expr),*) => (
            ( {
                let s = sb_itchy::stack::StackBuilder::new();
                $(
                    let s = s.next($x);
                )*
                s
            } )
        );
        // <$([<$arg_name:camel>]: BlockGeneratorInto<$arg_type>),*>
    }
}

pub mod blocks_wrapper {
    use sb_itchy::{
        block::{BlockFieldBuilder, BlockInputBuilder, FieldKind},
        blocks,
        stack::StackBuilder,
    };

    use super::block_generator_into::BlockGeneratorInto;
    use paste::paste;

    type Bfb = BlockFieldBuilder;
    type Bib = BlockInputBuilder;

    macro_rules! redefine_stack {
        ($func_name:ident ( $($arg_name:ident : $arg_type:ty),* )) => {
            paste! {
                #[inline]
                pub fn $func_name<$([<$arg_name:camel>]: BlockGeneratorInto<$arg_type>),*>($($arg_name: [<$arg_name:camel>]),*) -> StackBuilder {
                    blocks::$func_name( $($arg_name.to()),* )
                }
            }
        };
    }

    macro_rules! redefine_input {
        ($func_name:ident ( $($arg_name:ident : $arg_type:ty),* )) => {
            paste! {
                #[inline]
                pub fn $func_name<$([<$arg_name:camel>]: BlockGeneratorInto<$arg_type>),*>($($arg_name: [<$arg_name:camel>]),*) -> Bib {
                    Bib::stack(blocks::$func_name( $($arg_name.to()),* ))
                }
            }
        };
    }

    // Control =====================================================================
    redefine_stack!(wait(duration: Bib));
    redefine_stack!(repeat(times: Bib, to_repeat: Option<Bib>));
    redefine_stack!(forever(to_repeat: Option<Bib>));
    redefine_stack!(if_(condition: Bib, if_true: Option<Bib>));
    redefine_stack!(if_else(condition: Bib, if_true: Option<Bib>, if_false: Option<Bib>));
    redefine_stack!(wait_until(condition: Bib));
    redefine_stack!(repeat_until(condition: Bib, to_repeat: Option<Bib>));
    redefine_stack!(stop(stop_option: Bfb, has_next: bool));
    redefine_stack!(when_i_start_as_a_clone());
    redefine_stack!(create_clone_of(sprite: Bib));
    redefine_stack!(create_clone_of_menu(sprite: Bfb));
    redefine_stack!(delete_this_clone());

    // Event =======================================================================
    redefine_stack!(when_flag_clicked());
    redefine_stack!(when_key_pressed(key: Bfb));
    redefine_stack!(when_this_sprite_clicked());
    redefine_stack!(when_backdrop_switches_to(backdrop: Bfb));
    redefine_stack!(when_greater_than(variable: Bfb, value: Bib));
    redefine_stack!(when_broadcast_received(broadcast: Bfb));
    redefine_stack!(broadcast(broadcast: Bib));
    redefine_stack!(broadcast_and_wait(broadcast: Bib));

    // Looks =======================================================================
    redefine_stack!(think(message: Bib));
    redefine_stack!(think_for_secs(message: Bib, secs: Bib));
    redefine_stack!(say(message: Bib));
    redefine_stack!(say_for_secs(message: Bib, secs: Bib));
    redefine_stack!(switch_costume_to(costume: Bib));
    redefine_input!(costume_menu(costume: Bfb));
    redefine_stack!(next_costume());
    redefine_stack!(switch_backdrop_to(backdrop: Bib));
    redefine_input!(backdrop_menu(backdrop: Bfb));
    redefine_stack!(next_backdrop());
    redefine_stack!(change_size_by(by: Bib));
    redefine_stack!(set_size_to(to: Bib));
    redefine_stack!(change_looks_effect_by(effect: Bfb, by: Bib));
    redefine_stack!(set_looks_effect_to(effect: Bfb, to: Bib));
    redefine_stack!(clear_graphic_effects());
    redefine_stack!(show());
    redefine_stack!(hide());
    redefine_stack!(go_to_layer(layer: Bfb));
    redefine_stack!(change_layer(layer: Bfb, by: Bib));
    redefine_input!(costume(return_type: Bfb));
    redefine_input!(backdrop(return_type: Bfb));
    redefine_input!(size());

    // Motion ======================================================================
    redefine_stack!(move_steps(steps: Bib));
    redefine_stack!(turn_right(degress: Bib));
    redefine_stack!(turn_left(degress: Bib));
    redefine_stack!(go_to(to: Bib));
    redefine_input!(go_to_menu(to: Bfb));
    redefine_stack!(goto_xy(x: Bib, y: Bib));
    redefine_stack!(glide_to(duration_secs: Bib, to: Bib));
    redefine_input!(glide_to_menu(to: Bfb));
    redefine_stack!(glide_to_xy(dur: Bib, x: Bib, y: Bib));
    redefine_stack!(point_in_direction(direction: Bib));
    redefine_stack!(point_towards(towards: Bib));
    redefine_input!(point_towards_menu(towards: Bfb));
    redefine_stack!(set_x(x: Bib));
    redefine_stack!(set_y(y: Bib));
    redefine_stack!(change_x_by(by: Bib));
    redefine_stack!(change_y_by(by: Bib));
    redefine_stack!(if_on_edge_bounce());
    redefine_stack!(set_rotation_style(style: Bfb));
    redefine_input!(x_position());
    redefine_input!(y_position());
    redefine_input!(direction());

    // Operators ===================================================================
    redefine_input!(add(lhs: Bib, rhs: Bib));
    redefine_input!(sub(lhs: Bib, rhs: Bib));
    redefine_input!(mul(lhs: Bib, rhs: Bib));
    redefine_input!(div(lhs: Bib, rhs: Bib));
    redefine_input!(random(from: Bib, to: Bib));
    redefine_input!(less_than(lhs: Bib, rhs: Bib));
    redefine_input!(greater_than(lhs: Bib, rhs: Bib));
    redefine_input!(equals(lhs: Bib, rhs: Bib));
    redefine_input!(and(a: Bib, b: Bib));
    redefine_input!(or(a: Bib, b: Bib));
    redefine_input!(not(val: Bib));
    redefine_input!(join(a: Bib, b: Bib));
    redefine_input!(letter_of(idx: Bib, text: Bib));
    redefine_input!(length_of(text: Bib));
    redefine_input!(contains(text: Bib, contains: Bib));
    redefine_input!(modulo(dividend: Bib, divisor: Bib));
    redefine_input!(round(val: Bib));
    redefine_input!(math_op(op: Bfb, val: Bib));

    // Sensing =====================================================================
    redefine_input!(touching(what: Bib));
    redefine_input!(touching_menu(what: Bfb));
    redefine_input!(touching_color(color: Bib));
    redefine_input!(color_touching_color(color_a: Bib, color_b: Bib));
    redefine_input!(distance_to(what: Bib));
    redefine_input!(distance_to_menu(what: Bfb));
    redefine_stack!(ask_and_wait(prompt_message: Bib));
    redefine_input!(answer());
    redefine_input!(key_pressed(key: Bib));
    redefine_input!(key_menu(key: Bib));
    redefine_input!(mouse_down());
    redefine_input!(mouse_x());
    redefine_input!(mouse_y());
    redefine_stack!(set_drag_mode(mode: Bfb));
    redefine_input!(loudness());
    redefine_input!(timer());
    redefine_stack!(reset_timer());
    redefine_input!(var_of(var: Bfb, what: Bib));
    redefine_input!(var_of_object_menu(what: Bfb));
    redefine_input!(current_datetime(format: Bfb));
    redefine_input!(days_since_2000());
    redefine_input!(username());

    // Sound =======================================================================
    redefine_stack!(play_sound_until_done(sound: Bib));
    redefine_stack!(play_sound(sound: Bib));
    redefine_input!(sound_menu(sound: Bfb));
    redefine_stack!(stop_all_sound());
    redefine_stack!(change_sound_effect_by(effect: Bfb, by: Bib));
    redefine_stack!(set_sound_effect_to(effect: Bfb, to: Bib));
    redefine_stack!(clear_sound_effects());
    redefine_stack!(change_volume_by(by: Bib));
    redefine_stack!(set_volume_to(volume: Bib));
    redefine_input!(volume());

    // Data ========================================================================
    redefine_input!(sprite_var(name: String));
    redefine_input!(sprite_list(name: String));
    redefine_input!(global_var(name: String));
    redefine_input!(global_list(name: String));
    redefine_stack!(set_var_to(var: Bfb, to: Bib));
    redefine_stack!(change_var_by(var: Bfb, by: Bib));
    redefine_stack!(show_var(var: Bfb));
    redefine_stack!(hide_var(var: Bfb));
    redefine_stack!(add_to_list(list: Bfb, item: Bib));
    redefine_stack!(delete_in_list(list: Bfb, idx: Bib));
    redefine_stack!(delete_all_in_list(list: Bfb));
    redefine_stack!(insert_in_list(list: Bfb, idx: Bib, item: Bib));
    redefine_stack!(replace_in_list(list: Bfb, idx: Bib, item: Bib));
    redefine_input!(item_in_list(list: Bfb, idx: Bib));
    redefine_input!(count_of_item_in_list(list: Bfb, item: Bib));
    redefine_input!(length_of_list(list: Bfb));
    redefine_input!(list_contains(list: Bfb, item: Bib));
    redefine_stack!(show_list(list: Bfb));
    redefine_stack!(hide_list(list: Bfb));

    macro_rules! var_wrapper {
        ($func_name:ident, $kind:ident) => {
            #[inline]
            pub fn $func_name<S: Into<String>>(name: S) -> Bfb {
                Bfb::new_with_kind(name.into(), FieldKind::$kind)
            }
        };
    }
    var_wrapper!(global_var_menu, GlobalVariable);
    var_wrapper!(global_list_menu, GlobalList);
    var_wrapper!(sprite_var_menu, SpriteVariable);
    var_wrapper!(sprite_list_menu, SpriteList);

    // My Blocks ========================================================================
    pub use blocks::define_custom_block;
    pub use blocks::call_custom_block;
    redefine_input!(custom_block_var_string_number(name: String));
    redefine_input!(custom_block_var_boolean(name: String));
    pub fn always_true() -> Bib {
        Bib::stack(StackBuilder::start({
        let b = sb_itchy::block::BlockNormalBuilder::new(sb_itchy::opcode::StandardOpCode::operator_not);
        b
    }))
    }

    // Translate ========================================================================
    redefine_input!(translate_to(text: Bib, lang: String));
    redefine_input!(get_viewer_language());
}
