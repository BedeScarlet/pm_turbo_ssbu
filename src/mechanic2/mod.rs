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
    bitflags::bitflags,
};

pub unsafe extern "C" fn turbo_mode(fighter: &mut L2CFighterCommon) {

    let boma = fighter.module_accessor;

    let is_hitstop = StopModule::is_stop(boma);
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    let aerial_kind = ControlModule::get_attack_air_kind(boma);

    if CancelModule::is_enable_cancel(boma) 
        || is_hitstop 
        || !AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
            return;
    }

    if (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_N && motion_kind != smash::hash40("attack_air_n"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F && motion_kind != smash::hash40("attack_air_f"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_B && motion_kind != smash::hash40("attack_air_b"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_HI && motion_kind != smash::hash40("attack_air_hi"))
    || (aerial_kind == *FIGHTER_COMMAND_ATTACK_AIR_KIND_LW && motion_kind != smash::hash40("attack_air_lw")) {
        CancelModule::enable_cancel(boma);
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