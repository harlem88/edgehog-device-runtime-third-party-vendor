// automatically generated by rust-bindgen with manual adjustments

use super::{c_char, c_int, c_void, clockid_t, pid_t, siginfo_t, signalfd_siginfo};

#[allow(non_camel_case_types)]
pub enum sd_event {}
#[allow(non_camel_case_types)]
pub enum sd_event_source {}

pub const SD_EVENT_OFF: i32 = 0;
pub const SD_EVENT_ON: i32 = 1;
pub const SD_EVENT_ONESHOT: i32 = -1;

pub const SD_EVENT_INITIAL: u32 = 0;
pub const SD_EVENT_ARMED: u32 = 1;
pub const SD_EVENT_PENDING: u32 = 2;
pub const SD_EVENT_RUNNING: u32 = 3;
pub const SD_EVENT_EXITING: u32 = 4;
pub const SD_EVENT_FINISHED: u32 = 5;

pub const SD_EVENT_PRIORITY_IMPORTANT: i32 = -100;
pub const SD_EVENT_PRIORITY_NORMAL: i32 = 0;
pub const SD_EVENT_PRIORITY_IDLE: i32 = 100;

#[allow(non_camel_case_types)]
pub type sd_event_handler_t =
    Option<unsafe extern "C" fn(s: *mut sd_event_source, userdata: *mut c_void) -> c_int>;
#[allow(non_camel_case_types)]
pub type sd_event_io_handler_t = Option<
    unsafe extern "C" fn(
        s: *mut sd_event_source,
        fd: c_int,
        revents: u32,
        userdata: *mut c_void,
    ) -> c_int,
>;
#[allow(non_camel_case_types)]
pub type sd_event_time_handler_t = Option<
    unsafe extern "C" fn(s: *mut sd_event_source, usec: u64, userdata: *mut c_void) -> c_int,
>;
#[allow(non_camel_case_types)]
pub type sd_event_signal_handler_t = Option<
    unsafe extern "C" fn(
        s: *mut sd_event_source,
        si: *const signalfd_siginfo,
        userdata: *mut c_void,
    ) -> c_int,
>;
#[allow(non_camel_case_types)]
pub type sd_event_child_handler_t = Option<
    unsafe extern "C" fn(
        s: *mut sd_event_source,
        si: *const siginfo_t,
        userdata: *mut c_void,
    ) -> c_int,
>;
extern "C" {
    pub fn sd_event_default(e: *mut *mut sd_event) -> c_int;
    pub fn sd_event_new(e: *mut *mut sd_event) -> c_int;
    pub fn sd_event_ref(e: *mut sd_event) -> *mut sd_event;
    pub fn sd_event_unref(e: *mut sd_event) -> *mut sd_event;
    pub fn sd_event_add_io(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        fd: c_int,
        events: u32,
        callback: sd_event_io_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_time(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        clock: clockid_t,
        usec: u64,
        accuracy: u64,
        callback: sd_event_time_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_signal(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        sig: c_int,
        callback: sd_event_signal_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_child(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        pid: pid_t,
        options: c_int,
        callback: sd_event_child_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_defer(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        callback: sd_event_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_post(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        callback: sd_event_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_add_exit(
        e: *mut sd_event,
        s: *mut *mut sd_event_source,
        callback: sd_event_handler_t,
        userdata: *mut c_void,
    ) -> c_int;
    pub fn sd_event_prepare(e: *mut sd_event) -> c_int;
    pub fn sd_event_wait(e: *mut sd_event, timeout: u64) -> c_int;
    pub fn sd_event_dispatch(e: *mut sd_event) -> c_int;
    pub fn sd_event_run(e: *mut sd_event, timeout: u64) -> c_int;
    pub fn sd_event_loop(e: *mut sd_event) -> c_int;
    pub fn sd_event_exit(e: *mut sd_event, code: c_int) -> c_int;
    pub fn sd_event_now(e: *mut sd_event, clock: clockid_t, usec: *mut u64) -> c_int;
    pub fn sd_event_get_fd(e: *mut sd_event) -> c_int;
    pub fn sd_event_get_state(e: *mut sd_event) -> c_int;
    pub fn sd_event_get_tid(e: *mut sd_event, tid: *mut pid_t) -> c_int;
    pub fn sd_event_get_exit_code(e: *mut sd_event, code: *mut c_int) -> c_int;
    pub fn sd_event_set_watchdog(e: *mut sd_event, b: c_int) -> c_int;
    pub fn sd_event_get_watchdog(e: *mut sd_event) -> c_int;
    pub fn sd_event_source_ref(s: *mut sd_event_source) -> *mut sd_event_source;
    pub fn sd_event_source_unref(s: *mut sd_event_source) -> *mut sd_event_source;
    pub fn sd_event_source_get_event(s: *mut sd_event_source) -> *mut sd_event;
    pub fn sd_event_source_get_userdata(s: *mut sd_event_source) -> *mut c_void;
    pub fn sd_event_source_set_userdata(
        s: *mut sd_event_source,
        userdata: *mut c_void,
    ) -> *mut c_void;
    pub fn sd_event_source_set_description(
        s: *mut sd_event_source,
        description: *const c_char,
    ) -> c_int;
    pub fn sd_event_source_get_description(
        s: *mut sd_event_source,
        description: *mut *const c_char,
    ) -> c_int;
    pub fn sd_event_source_set_prepare(
        s: *mut sd_event_source,
        callback: sd_event_handler_t,
    ) -> c_int;
    pub fn sd_event_source_get_pending(s: *mut sd_event_source) -> c_int;
    pub fn sd_event_source_get_priority(s: *mut sd_event_source, priority: *mut i64) -> c_int;
    pub fn sd_event_source_set_priority(s: *mut sd_event_source, priority: i64) -> c_int;
    pub fn sd_event_source_get_enabled(s: *mut sd_event_source, enabled: *mut c_int) -> c_int;
    pub fn sd_event_source_set_enabled(s: *mut sd_event_source, enabled: c_int) -> c_int;
    pub fn sd_event_source_get_io_fd(s: *mut sd_event_source) -> c_int;
    pub fn sd_event_source_set_io_fd(s: *mut sd_event_source, fd: c_int) -> c_int;
    pub fn sd_event_source_get_io_events(s: *mut sd_event_source, events: *mut u32) -> c_int;
    pub fn sd_event_source_set_io_events(s: *mut sd_event_source, events: u32) -> c_int;
    pub fn sd_event_source_get_io_revents(s: *mut sd_event_source, revents: *mut u32) -> c_int;
    pub fn sd_event_source_get_time(s: *mut sd_event_source, usec: *mut u64) -> c_int;
    pub fn sd_event_source_set_time(s: *mut sd_event_source, usec: u64) -> c_int;
    pub fn sd_event_source_get_time_accuracy(s: *mut sd_event_source, usec: *mut u64) -> c_int;
    pub fn sd_event_source_set_time_accuracy(s: *mut sd_event_source, usec: u64) -> c_int;
    pub fn sd_event_source_get_time_clock(s: *mut sd_event_source, clock: *mut clockid_t) -> c_int;
    pub fn sd_event_source_get_signal(s: *mut sd_event_source) -> c_int;
    pub fn sd_event_source_get_child_pid(s: *mut sd_event_source, pid: *mut pid_t) -> c_int;
}
