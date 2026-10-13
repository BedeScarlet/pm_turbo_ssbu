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

pub fn is_grab(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let status_kind = StatusModule::status_kind(boma);

        if status_kind == *FIGHTER_STATUS_KIND_CATCH
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_WAIT 
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_PULL
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_JUMP
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_DASH_PULL
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_TURN 
        || status_kind == *FIGHTER_STATUS_KIND_CATCH_CUT {
            return true;
        } else {
            return false;
        }
    }
}

pub fn allow_grabcancel(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        let status_kind = StatusModule::status_kind(boma);
        if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
            if !is_grab(fighter) {
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                }
            }
        }
    }
}

pub fn is_sticktilt(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);
        let stick_x = ControlModule::get_stick_x(boma);
        let stick_y = ControlModule::get_stick_y(boma);
        let s3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_s3_stick_x"));
        let hi3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_hi3_stick_y"));
        let lw3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_lw3_stick_y"));

        if stick_x >= s3_val || stick_y >= hi3_val || stick_y <= lw3_val {
            return true;
        }
        return false;
    }
}

pub fn is_sub_sticktilt(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);
        let stick_x = ControlModule::get_sub_stick_x(boma);
        let stick_y = ControlModule::get_sub_stick_y(boma);
        let s3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_s3_stick_x"));
        let hi3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_hi3_stick_y"));
        let lw3_val = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_lw3_stick_y"));

        if stick_x >= s3_val || stick_y >= hi3_val || stick_y <= lw3_val {
            return true;
        }
        return false;
    }
}


pub fn is_flick_fsmash(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);
        let stick_x = ControlModule::get_stick_x(boma);
        let flick_x = ControlModule::get_flick_x(boma);
        let dash_stick = WorkModule::get_param_float(boma, hash40("common"), hash40("dash_stick_x"));
        let s4_flick = FighterControlModuleImpl::get_param_dash_s4_frame(boma) as i32;

        if stick_x.abs() >= dash_stick && flick_x <= s4_flick {
            return true;
        }
        return false;
    }
}

pub fn is_flick_usmash(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);
        let stick_y = ControlModule::get_stick_y(boma);
        let flick_y = ControlModule::get_flick_y(boma);
        let hi4_stick = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_hi4_stick_y"));
        let hi4_flick = FighterControlModuleImpl::get_param_attack_hi4_flick_y(boma) as i32;

        if flick_y <= hi4_flick && stick_y.abs() >= hi4_stick {
            return true;
        }
        return false;
    }
}

pub fn is_flick_dsmash(fighter: &mut L2CFighterCommon) -> bool {
    unsafe {
        let boma = fighter.module_accessor;
        let command_kind1 = ControlModule::get_command_flag_cat(boma, 0);
        let stick_y = ControlModule::get_stick_y(boma);
        let flick_y = ControlModule::get_flick_y(boma);
        let lw4_stick = WorkModule::get_param_float(boma, hash40("common"), hash40("attack_lw4_stick_y"));
        let lw4_flick = FighterControlModuleImpl::get_param_attack_lw4_flick_y(boma) as i32;

        if flick_y <= lw4_flick && stick_y.abs() >= lw4_stick {
            return true;
        }
        return false;
    }
}