// rustium/main.rs
//
// Copyright (c) 2014 Mozilla Foundation

extern crate libc;

use libc::{c_char, c_int, c_void, size_t, uint16_t};
use std::mem;
use std::ptr;

// cef_process_id_t
const PID_BROWSER: c_int = 0;
const PID_RENDERER: c_int = 1;

// cef_state_t
const STATE_DEFAULT: c_int = 0;
const STATE_ENABLED: c_int = 1;
const STATE_DISABLED: c_int = 2;

#[repr(C)]
struct cef_app_t {
    base: cef_base_t,
    on_before_command_line_processing: extern "C" fn(this: *mut cef_app_t,
                                                     process_type: *const cef_string_t,
                                                     command_line: *const cef_command_line_t),
    on_register_custom_schemes: extern "C" fn(this: *mut cef_app_t,
                                              registrar: *mut cef_scheme_registrar_t),
    get_resource_bundle_handler: extern "C" fn(this: *mut cef_app_t)
                                               -> *mut cef_resource_bundle_handler_t,
    get_browser_process_handler: extern "C" fn(this: *mut cef_app_t)
                                               -> *mut cef_browser_process_handler_t,
    get_render_process_handler: extern "C" fn(this: *mut cef_app_t)
                                              -> *mut cef_render_process_handler_t,
}

#[repr(C)]
struct cef_base_t {
    size: size_t,
    add_ref: extern "C" fn(this: *mut cef_base_t),
    release: extern "C" fn(this: *mut cef_base_t) -> c_int,
    has_one_ref: extern "C" fn(this: *mut cef_base_t) -> c_int,
}

type cef_browser_process_handler_t = c_void;
type cef_browser_t = c_void;

#[repr(C)]
struct cef_browser_settings_t {
    size: size_t,
    windowless_frame_rate: c_int,
    standard_font_family: cef_string_t,
    fixed_font_family: cef_string_t,
    serif_font_family: cef_string_t,
    sans_serif_font_family: cef_string_t,
    cursive_font_family: cef_string_t,
    fantasy_font_family: cef_string_t,
    default_font_size: c_int,
    default_fixed_font_size: c_int,
    minimum_font_size: c_int,
    minimum_logical_font_size: c_int,
    default_encoding: cef_string_t,
    remote_fonts: cef_state_t,
    javascript: cef_state_t,
    javascript_open_windows: cef_state_t,
    javascript_close_windows: cef_state_t,
    javascript_access_clipboard: cef_state_t,
    javascript_dom_paste: cef_state_t,
    caret_browsing: cef_state_t,
    java: cef_state_t,
    plugins: cef_state_t,
    universal_access_from_file_urls: cef_state_t,
    file_access_from_file_urls: cef_state_t,
    web_security: cef_state_t,
    image_loading: cef_state_t,
    image_shrink_standalone_to_fit: cef_state_t,
    text_area_resize: cef_state_t,
    tab_to_links: cef_state_t,
    local_storage: cef_state_t,
    databases: cef_state_t,
    application_cache: cef_state_t,
    webgl: cef_state_t,
    background_color: cef_state_t,
}

impl cef_browser_settings_t {
    fn new() -> cef_browser_settings_t {
        cef_browser_settings_t {
            size: mem::size_of::<cef_browser_settings_t>() as size_t,
            windowless_frame_rate: 0,
            standard_font_family: cef_string_utf16_t::empty(),
            fixed_font_family: cef_string_utf16_t::empty(),
            serif_font_family: cef_string_utf16_t::empty(),
            sans_serif_font_family: cef_string_utf16_t::empty(),
            cursive_font_family: cef_string_utf16_t::empty(),
            fantasy_font_family: cef_string_utf16_t::empty(),
            default_font_size: 12,
            default_fixed_font_size: 12,
            minimum_font_size: 0,
            minimum_logical_font_size: 0,
            default_encoding: cef_string_utf16_t::empty(),
            remote_fonts: 0,
            javascript: 0,
            javascript_open_windows: 0,
            javascript_close_windows: 0,
            javascript_access_clipboard: 0,
            javascript_dom_paste: 0,
            caret_browsing: 0,
            java: 0,
            plugins: 0,
            universal_access_from_file_urls: 0,
            file_access_from_file_urls: 0,
            web_security: 0,
            image_loading: 0,
            image_shrink_standalone_to_fit: 0,
            text_area_resize: 0,
            tab_to_links: 0,
            local_storage: 0,
            databases: 0,
            application_cache: 0,
            webgl: 0,
            background_color: 0,
        }
    }
}

type cef_char_t = u16;

#[repr(C)]
struct cef_client_t {
    base: cef_base_t,
    get_context_menu_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_context_menu_handler_t,
    get_dialog_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_dialog_handler_t,
    get_display_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_display_handler_t,
    get_download_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_download_handler_t,
    get_drag_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_drag_handler_t,
    get_focus_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_focus_handler_t,
    get_geolocation_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_geolocation_handler_t,
    get_jsdialog_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_jsdialog_handler_t,
    get_keyboard_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_keyboard_handler_t,
    get_life_span_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_life_span_handler_t,
    get_load_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_load_handler_t,
    get_render_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_render_handler_t,
    get_request_handler: extern "C" fn(*mut cef_client_t) -> *mut cef_request_handler_t,
    on_process_message_received: extern "C" fn(*mut cef_client_t,
                                               *mut cef_browser_t,
                                               cef_process_id_t,
                                               *mut cef_process_message_t)
                                               -> *mut cef_on_process_received_t,
}

type cef_color_t = u32;

type cef_command_line_t = c_void;

type cef_context_menu_handler_t = c_void;

type cef_dialog_handler_t = c_void;

type cef_display_handler_t = c_void;

type cef_download_handler_t = c_void;

type cef_drag_handler_t = c_void;

type cef_focus_handler_t = c_void;

type cef_geolocation_handler_t = c_void;

type cef_jsdialog_handler_t = c_void;

type cef_keyboard_handler_t = c_void;

type cef_life_span_handler_t = c_void;

type cef_load_handler_t = c_void;

type cef_log_severity_t = c_int;

#[repr(C)]
struct cef_main_args_t {
    argc: c_int,
    argv: *mut *mut c_char,
}

type cef_on_process_received_t = c_void;

type cef_process_id_t = c_int;

type cef_process_message_t = c_void;

type cef_render_handler_t = c_void;

type cef_render_process_handler_t = c_void;

type cef_request_context_t = c_void;

type cef_request_handler_t = c_void;

type cef_resource_bundle_handler_t = c_void;

type cef_scheme_registrar_t = c_void;

struct cef_settings_t {
    size: size_t,
    single_process: c_int,
    no_sandbox: c_int,
    browser_subprocess_path: cef_string_t,
    multi_threaded_message_loop: c_int,
    windowless_rendering_enabled: c_int,
    command_line_args_disabled: c_int,
    cache_path: cef_string_t,
    persist_session_cookies: c_int,
    user_agent: cef_string_t,
    product_version: cef_string_t,
    locale: cef_string_t,
    log_file: cef_string_t,
    log_severity: cef_log_severity_t,
    javascript_flags: cef_string_t,
    resources_dir_path: cef_string_t,
    locales_dir_path: cef_string_t,
    pack_loading_disabled: c_int,
    remote_debugging_port: c_int,
    uncaught_exception_stack_size: c_int,
    context_safety_implementation: c_int,
    ignore_certificate_errors: c_int,
    background_color: cef_color_t,
}

impl cef_settings_t {
    fn new() -> cef_settings_t {
        cef_settings_t {
            size: mem::size_of::<cef_settings_t>() as size_t,
            single_process: 0,
            no_sandbox: 0,
            browser_subprocess_path: cef_string_utf16_t::empty(),
            multi_threaded_message_loop: 0,
            windowless_rendering_enabled: 0,
            command_line_args_disabled: 0,
            cache_path: cef_string_utf16_t::empty(),
            persist_session_cookies: 0,
            user_agent: cef_string_utf16_t::empty(),
            product_version: cef_string_utf16_t::empty(),
            locale: cef_string_utf16_t::empty(),
            log_file: cef_string_utf16_t::empty(),
            log_severity: 0,
            javascript_flags: cef_string_utf16_t::empty(),
            resources_dir_path: cef_string_utf16_t::empty(),
            locales_dir_path: cef_string_utf16_t::empty(),
            pack_loading_disabled: 0,
            remote_debugging_port: 0,
            uncaught_exception_stack_size: 0,
            context_safety_implementation: 0,
            ignore_certificate_errors: 0,
            background_color: 0,
        }
    }
}

type cef_state_t = i32;

type cef_string_t = cef_string_utf16_t;

#[repr(C)]
struct cef_string_utf16_t {
    string: *mut u16,
    length: size_t,
    dtor: extern "C" fn(string: *mut u16),
}

impl cef_string_utf16_t {
    fn empty() -> cef_string_utf16_t {
        unsafe {
            cef_string_t {
                string: ptr::null_mut(),
                length: 0,
                dtor: mem::transmute(0u),
            }
        }
    }
}

type cef_window_handle_t = *mut c_void;

#[repr(C)]
struct cef_window_info_t {
    window_name: cef_string_t,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    hidden: c_int,
    parent_view: cef_window_handle_t,
    windowless_rendering_enabled: c_int,
    transparent_painting_enabled: c_int,
    view: cef_window_handle_t,
}

impl cef_window_info_t {
    fn new() -> cef_window_info_t {
        cef_window_info_t {
            window_name: cef_string_utf16_t::empty(),
            x: 0,
            y: 0,
            width: 640,
            height: 480,
            hidden: 0,
            parent_view: ptr::null_mut(),
            windowless_rendering_enabled: 0,
            transparent_painting_enabled: 0,
            view: ptr::null_mut(),
        }
    }
}

#[link(name="Chromium Embedded Framework", kind="framework")]
extern {
    fn cef_browser_host_create_browser(windowInfo: *const cef_window_info_t,
                                       client: *mut cef_client_t,
                                       url: *const cef_string_t,
                                       settings: *const cef_browser_settings_t,
                                       request_context: *const cef_request_context_t)
                                       -> c_int;
    fn cef_execute_process(args: *const cef_main_args_t,
                           application: *mut cef_app_t,
                           windows_sandbox_info: *mut c_void);
    fn cef_initialize(args: *const cef_main_args_t,
                      settings: *const cef_settings_t,
                      application: *mut cef_app_t,
                      windows_sandbox_info: *mut c_void)
                      -> c_int;
    fn cef_run_message_loop();
    fn cef_string_utf8_to_utf16(src: *const c_char,
                                src_len: size_t,
                                output: *mut cef_string_utf16_t)
                                -> c_int;
    fn cef_shutdown();
}

fn main() {
    unsafe {
        let main_args = cef_main_args_t {
            argc: 0,
            argv: ptr::null_mut(),
        };

        let mut settings = cef_settings_t::new();
        settings.single_process = 1;

        let mut app: cef_app_t = cef_app_t {
            base: cef_base_t {
                size: mem::size_of::<cef_app_t>() as size_t,
                add_ref: mem::transmute(0u),
                release: mem::transmute(0u),
                has_one_ref: mem::transmute(0u),
            },
            on_before_command_line_processing: mem::transmute(0u),
            on_register_custom_schemes: mem::transmute(0u),
            get_resource_bundle_handler: mem::transmute(0u),
            get_browser_process_handler: mem::transmute(0u),
            get_render_process_handler: mem::transmute(0u),
        };

        cef_initialize(&main_args, &settings, &mut app, ptr::null_mut());

        let code = cef_execute_process(&main_args, &mut app, ptr::null_mut());

        let mut url = cef_string_t {
            string: ptr::null_mut(),
            length: 0,
            dtor: mem::transmute(0u),
        };
        let default_url = "http://mozilla.org/".to_c_str();
        cef_string_utf8_to_utf16(default_url.as_ptr(), default_url.len() as size_t, &mut url);

        let window_info = cef_window_info_t::new();

        let mut client = cef_client_t {
            base: cef_base_t {
                size: mem::size_of::<cef_client_t>() as size_t,
                add_ref: mem::transmute(0u),
                release: mem::transmute(0u),
                has_one_ref: mem::transmute(0u),
            },
            get_context_menu_handler: mem::transmute(0u),
            get_dialog_handler: mem::transmute(0u),
            get_display_handler: mem::transmute(0u),
            get_download_handler: mem::transmute(0u),
            get_drag_handler: mem::transmute(0u),
            get_focus_handler: mem::transmute(0u),
            get_geolocation_handler: mem::transmute(0u),
            get_jsdialog_handler: mem::transmute(0u),
            get_keyboard_handler: mem::transmute(0u),
            get_life_span_handler: mem::transmute(0u),
            get_load_handler: mem::transmute(0u),
            get_render_handler: mem::transmute(0u),
            get_request_handler: mem::transmute(0u),
            on_process_message_received: mem::transmute(0u),
        };

        let browser_settings = cef_browser_settings_t::new();

        let result = cef_browser_host_create_browser(&window_info,
                                                     &mut client,
                                                     &url,
                                                     &browser_settings,
                                                     ptr::null());

        cef_run_message_loop();

        cef_shutdown();
    }
}

