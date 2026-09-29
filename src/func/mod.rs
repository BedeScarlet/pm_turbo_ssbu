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

pub fn allow_grabcancel(fighter: &mut L2CFighterCommon) {
unsafe {
    let boma = fighter.module_accessor;
    if StatusModule::situation_kind(boma) == SITUATION_KIND_GROUND {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_CATCH, false);
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