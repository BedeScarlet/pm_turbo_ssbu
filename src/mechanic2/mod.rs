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
    let status_prev_kind = StatusModule::prev_status_kind(boma, 0);
    let motion_kind = MotionModule::motion_kind(boma);
    let aerial_kind = ControlModule::get_attack_air_kind(boma);
    let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);

    if CancelModule::is_enable_cancel(boma) 
        || if_hitlag 
        || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            return;
    }

    // Aerials

    // enable_cancel first looks at the next move input you do.
    // if the move is normally prevented from coming out (i.e. you're doing another move and it hasn't ended yet)
    // ... enable_cancel allows that next move to come out anyway.
    // We have to check for the command, but only when we know the move isn't already being performed. hence we use MotionModule to keep track of the animation
    // todo: account for multipart aerials (sora, bayo)
    if (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N && motion_kind != smash::hash40("attack_air_n"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F && motion_kind != smash::hash40("attack_air_f"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B && motion_kind != smash::hash40("attack_air_b"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI && motion_kind != smash::hash40("attack_air_hi"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW && motion_kind != smash::hash40("attack_air_lw")) {
        CancelModule::enable_cancel(boma);
    }

    // Ground Normals
    if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND { // game will try to make you do grounded normals in midair if we dont do this
        // Jab 1 / 2 / 3 cannot cancel into each other, but can be cancelled by dash and walk.
        // it'll take some extra effort to have jab get interrupted by walk/dash while preventing it from cancelling into itself
        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
            if ![
                smash::hash40("attack_11"), 
                smash::hash40("attack_12"), 
                smash::hash40("attack_13"), 
                smash::hash40("attack_100start"), 
                smash::hash40("attack_100"), 
                smash::hash40("attack_100end")]
                .contains(&motion_kind) {
                    CancelModule::enable_cancel(boma);
            }
        }
        
        // Dash attack cancels into walk if the input is held.
        // letting it cancel into itself isn't really a problem.
        // Maybe do something about the transition, though. interp looks fugly  
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH {
            CancelModule::enable_cancel(boma);
        }

        if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
            if ![
                smash::hash40("attack_s3_s"), 
                smash::hash40("attack_s3_hi"), 
                smash::hash40("attack_s3_lw"), 
                smash::hash40("attack_s3_s2"), 
                smash::hash40("attack_s3_s3")]
                .contains(&motion_kind) {
                    CancelModule::enable_cancel(boma);
            }
        }

        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI3 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_S4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_HI4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, false);
        //     }
        // }
        // if (command_kind1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
        //     if status_kind != *FIGHTER_STATUS_KIND_ATTACK_LW4 {
        //         StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, false);
        //     }
        // }
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