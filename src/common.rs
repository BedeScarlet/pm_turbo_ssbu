use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{
            self, sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*},
    crate::func::*
};

pub unsafe extern "C" fn turbo_mode(fighter: &mut L2CFighterCommon) {

    let boma = fighter.module_accessor;

    let if_hitlag: bool;
    let hitlag_frame = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
    if hitlag_frame > 0 {
        if_hitlag = true;
    } else {
        if_hitlag = false
    }

    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let aerial_kind = ControlModule::get_attack_air_kind(boma);
    let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);

    if if_hitlag 
    || CancelModule::is_enable_cancel(boma)
    || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        return;
    }

    if [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_ATTACK
        ].contains(&status_kind) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        fighter.sub_transition_group_check_ground_jump();
        fighter.sub_transition_group_check_air_jump_aerial();
    }
    // Air Moves
    // todo: account for multipart aerials (sora, not bayo though idk why)
    if StatusModule::situation_kind(boma) == SITUATION_KIND_AIR {
        if (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N && motion_kind != smash::hash40("attack_air_n"))
        || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F && motion_kind != smash::hash40("attack_air_f"))
        || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B && motion_kind != smash::hash40("attack_air_b"))
        || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI && motion_kind != smash::hash40("attack_air_hi"))
        || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW && motion_kind != smash::hash40("attack_air_lw")) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
            }
        }
    }

    // Ground Moves
    if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
        if is_grab(fighter) {
            return;
        }
        allow_grabcancel(fighter);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK)  
        && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            // this command flag genuinely runs anytime you use a move with the A button. i think
            let tltsms = [
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4
                ];
            if 
            status_kind != *FIGHTER_STATUS_KIND_ATTACK && 
            !is_sticktilt(fighter) && 
            !is_sub_sticktilt(fighter) {
                if 
                !tltsms.contains(&command_kind1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, false);
                }
            }
        }

        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            CancelModule::enable_cancel(boma);
        }

        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) 
        || (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && is_sticktilt(fighter)) 
        {
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 && (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) == 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
            }
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 && (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) == 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
                }
            }
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 && (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) == 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
                }
            }
        }

        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 && !ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SMASH) {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 && !ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SMASH) {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW3 && !ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_SMASH) {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        //     }
        // }

        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SMASH) 
        || (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) && (is_flick_usmash(fighter) || is_flick_fsmash(fighter) || is_flick_dsmash(fighter)))
        {
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S4 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
            }
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI4 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
            }
            if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
            }
        }

        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
        //     }
        // }

        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_N {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_S {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI, false);
            }
        }
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0 {
            if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, false);
            }
        }
    }

    if CancelModule::is_enable_cancel(boma) {
        if status_kind == *SITUATION_KIND_GROUND {
                fighter.sub_wait_ground_check_common(false.into());
            } else {
                fighter.sub_air_check_fall_common();
            }
    }

}

pub fn install() {
    Agent::new("fighter")
    .on_line(Main, turbo_mode)
    .install();
}