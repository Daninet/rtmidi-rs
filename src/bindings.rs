use std::os::raw::{c_char, c_int, c_uchar, c_uint};

#[doc = "! \\brief Wraps an RtMidi object for C function return statuses."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RtMidiWrapper {
    #[doc = "! The wrapped RtMidi object."]
    pub ptr: *mut c_void,
    pub data: *mut c_void,
    #[doc = "! True when the last function call was OK."]
    pub ok: bool,
    #[doc = "! If an error occurred (ok != true), set to an error message."]
    pub msg: *const c_char,
}

#[doc = "! \\brief Typedef for a generic RtMidi pointer."]
pub type RtMidiPtr = *mut RtMidiWrapper;

#[doc = "! \\brief Typedef for a generic RtMidiIn pointer."]
pub type RtMidiInPtr = *mut RtMidiWrapper;

#[doc = "! \\brief Typedef for a generic RtMidiOut pointer."]
pub type RtMidiOutPtr = *mut RtMidiWrapper;

#[doc = "< Search for a working compiled API."]
pub const RTMIDI_API_UNSPECIFIED: RtMidiApi = 0;

#[doc = "< Macintosh OS-X CoreMIDI API."]
pub const RTMIDI_API_MACOSX_CORE: RtMidiApi = 1;

#[doc = "< The Advanced Linux Sound Architecture API."]
pub const RTMIDI_API_LINUX_ALSA: RtMidiApi = 2;

#[doc = "< The Jack Low-Latency MIDI Server API."]
pub const RTMIDI_API_UNIX_JACK: RtMidiApi = 3;

#[doc = "< The Microsoft Multimedia MIDI API."]
pub const RTMIDI_API_WINDOWS_MM: RtMidiApi = 4;

#[doc = "< A compilable but non-functional API."]
pub const RTMIDI_API_RTMIDI_DUMMY: RtMidiApi = 5;

#[doc = "< W3C Web MIDI API."]
pub const RTMIDI_API_WEB_MIDI_API: RtMidiApi = 6;

#[doc = "< The Microsoft Universal Windows Platform MIDI API."]
pub const RTMIDI_API_WINDOWS_UWP: RtMidiApi = 7;

#[doc = "< The Android MIDI API."]
pub const RTMIDI_API_ANDROID: RtMidiApi = 8;

#[doc = "< Number of values in this enum."]
pub const RTMIDI_API_NUM: RtMidiApi = 9;

#[doc = "! \\brief MIDI API specifier arguments.  See \\ref RtMidi::Api."]
pub type RtMidiApi = c_int;

#[doc = "< A non-critical error."]
pub const RTMIDI_ERROR_WARNING: RtMidiErrorType = 0;

#[doc = "< A non-critical error which might be useful for debugging."]
pub const RTMIDI_ERROR_DEBUG_WARNING: RtMidiErrorType = 1;

#[doc = "< The default, unspecified error type."]
pub const RTMIDI_ERROR_UNSPECIFIED: RtMidiErrorType = 2;

#[doc = "< No devices found on system."]
pub const RTMIDI_ERROR_NO_DEVICES_FOUND: RtMidiErrorType = 3;

#[doc = "< An invalid device ID was specified."]
pub const RTMIDI_ERROR_INVALID_DEVICE: RtMidiErrorType = 4;

#[doc = "< An error occurred during memory allocation."]
pub const RTMIDI_ERROR_MEMORY_ERROR: RtMidiErrorType = 5;

#[doc = "< An invalid parameter was specified to a function."]
pub const RTMIDI_ERROR_INVALID_PARAMETER: RtMidiErrorType = 6;

#[doc = "< The function was called incorrectly."]
pub const RTMIDI_ERROR_INVALID_USE: RtMidiErrorType = 7;

#[doc = "< A system driver error occurred."]
pub const RTMIDI_ERROR_DRIVER_ERROR: RtMidiErrorType = 8;

#[doc = "< A system error occurred."]
pub const RTMIDI_ERROR_SYSTEM_ERROR: RtMidiErrorType = 9;

#[doc = "< A thread error occurred."]
pub const RTMIDI_ERROR_THREAD_ERROR: RtMidiErrorType = 10;

#[doc = "! \\brief Defined RtMidiError types. See \\ref RtMidiError::Type."]
pub type RtMidiErrorType = c_int;

#[doc = " \\brief The type of a RtMidi callback function.\n\n \\param timeStamp   The time at which the message has been received.\n \\param message     The midi message.\n \\param userData    Additional user data for the callback.\n\n See \\ref RtMidiIn::RtMidiCallback."]
pub type RtMidiCCallback = ::std::option::Option<
    unsafe extern "C" fn(
        timeStamp: f64,
        message: *const c_uchar,
        messageSize: usize,
        userData: *mut c_void,
    ),
>;

unsafe extern "C" {
    #[doc = " \\brief Return the current RtMidi version.\n See \\ref RtMidi::getVersion()."]
    pub fn rtmidi_get_version() -> *const c_char;

    #[doc = " \\brief Determine the available compiled MIDI APIs.\n\n If the given `apis` parameter is null, returns the number of available APIs.\n Otherwise, fill the given apis array with the RtMidi::Api values.\n\n \\param apis  An array or a null value.\n \\param apis_size  Number of elements pointed to by apis\n \\return number of items needed for apis array if apis==NULL, or\n         number of items written to apis array otherwise.  A negative\n         return value indicates an error.\n\n See \\ref RtMidi::getCompiledApi()."]
    pub fn rtmidi_get_compiled_api(apis: *mut RtMidiApi, apis_size: c_uint) -> c_int;

    #[doc = "! \\brief Return the name of a specified compiled MIDI API.\n! See \\ref RtMidi::getApiName()."]
    pub fn rtmidi_api_name(api: RtMidiApi) -> *const c_char;

    #[doc = "! \\brief Return the display name of a specified compiled MIDI API.\n! See \\ref RtMidi::getApiDisplayName()."]
    pub fn rtmidi_api_display_name(api: RtMidiApi) -> *const c_char;

    #[doc = "! \\brief Return the compiled MIDI API having the given name.\n! See \\ref RtMidi::getCompiledApiByName()."]
    pub fn rtmidi_compiled_api_by_name(name: *const c_char) -> RtMidiApi;

    #[doc = "! \\internal Report an error."]
    pub fn rtmidi_error(type_: RtMidiErrorType, errorString: *const c_char);

    #[doc = " \\brief Open a MIDI port.\n\n \\param port      Must be greater than 0\n \\param portName  Name for the application port.\n\n See RtMidi::openPort()."]
    pub fn rtmidi_open_port(device: RtMidiPtr, portNumber: c_uint, portName: *const c_char);

    #[doc = " \\brief Creates a virtual MIDI port to which other software applications can\n connect.\n\n \\param portName  Name for the application port.\n\n See RtMidi::openVirtualPort()."]
    pub fn rtmidi_open_virtual_port(device: RtMidiPtr, portName: *const c_char);

    #[doc = " \\brief Close a MIDI connection.\n See RtMidi::closePort()."]
    pub fn rtmidi_close_port(device: RtMidiPtr);

    #[doc = " \\brief Return the number of available MIDI ports.\n See RtMidi::getPortCount()."]
    pub fn rtmidi_get_port_count(device: RtMidiPtr) -> c_uint;

    #[doc = " \\brief Access a string identifier for the specified MIDI input port number.\n\n To prevent memory leaks a char buffer must be passed to this function.\n NULL can be passed as bufOut parameter, and that will write the required buffer length in the bufLen.\n\n See RtMidi::getPortName()."]
    pub fn rtmidi_get_port_name(
        device: RtMidiPtr,
        portNumber: c_uint,
        bufOut: *mut c_char,
        bufLen: *mut c_int,
    ) -> c_int;

    #[doc = "! \\brief Create a default RtMidiInPtr value, with no initialization."]
    pub fn rtmidi_in_create_default() -> RtMidiInPtr;

    #[doc = " \\brief Create a  RtMidiInPtr value, with given api, clientName and queueSizeLimit.\n\n  \\param api            An optional API id can be specified.\n  \\param clientName     An optional client name can be specified. This\n                        will be used to group the ports that are created\n                        by the application.\n  \\param queueSizeLimit An optional size of the MIDI input queue can be\n                        specified.\n\n See RtMidiIn::RtMidiIn()."]
    pub fn rtmidi_in_create(
        api: RtMidiApi,
        clientName: *const c_char,
        queueSizeLimit: c_uint,
    ) -> RtMidiInPtr;

    #[doc = "! \\brief Free the given RtMidiInPtr."]
    pub fn rtmidi_in_free(device: RtMidiInPtr);

    #[doc = "! \\brief Returns the MIDI API specifier for the given instance of RtMidiIn.\n! See \\ref RtMidiIn::getCurrentApi()."]
    pub fn rtmidi_in_get_current_api(device: RtMidiPtr) -> RtMidiApi;

    #[doc = "! \\brief Set a callback function to be invoked for incoming MIDI messages.\n! See \\ref RtMidiIn::setCallback()."]
    pub fn rtmidi_in_set_callback(
        device: RtMidiInPtr,
        callback: RtMidiCCallback,
        userData: *mut c_void,
    );

    #[doc = "! \\brief Cancel use of the current callback function (if one exists).\n! See \\ref RtMidiIn::cancelCallback()."]
    pub fn rtmidi_in_cancel_callback(device: RtMidiInPtr);

    #[doc = "! \\brief Specify whether certain MIDI message types should be queued or ignored during input.\n! See \\ref RtMidiIn::ignoreTypes()."]
    pub fn rtmidi_in_ignore_types(
        device: RtMidiInPtr,
        midiSysex: bool,
        midiTime: bool,
        midiSense: bool,
    );

    #[doc = " Fill the user-provided array with the data bytes for the next available\n MIDI message in the input queue and return the event delta-time in seconds.\n\n \\param message   Must point to a char* that is already allocated.\n                  SYSEX messages maximum size being 1024, a statically\n                  allocated array could\n                  be sufficient.\n \\param size      Is used to return the size of the message obtained.\n                  Must be set to the size of \\ref message when calling.\n\n See RtMidiIn::getMessage()."]
    pub fn rtmidi_in_get_message(
        device: RtMidiInPtr,
        message: *mut c_uchar,
        size: *mut usize,
    ) -> f64;

    #[doc = "! \\brief Create a default RtMidiInPtr value, with no initialization."]
    pub fn rtmidi_out_create_default() -> RtMidiOutPtr;

    #[doc = " \\brief Create a RtMidiOutPtr value, with given and clientName.\n\n  \\param api            An optional API id can be specified.\n  \\param clientName     An optional client name can be specified. This\n                        will be used to group the ports that are created\n                        by the application.\n\n See RtMidiOut::RtMidiOut()."]
    pub fn rtmidi_out_create(api: RtMidiApi, clientName: *const c_char) -> RtMidiOutPtr;

    #[doc = "! \\brief Free the given RtMidiOutPtr."]
    pub fn rtmidi_out_free(device: RtMidiOutPtr);

    #[doc = "! \\brief Returns the MIDI API specifier for the given instance of RtMidiOut.\n! See \\ref RtMidiOut::getCurrentApi()."]
    pub fn rtmidi_out_get_current_api(device: RtMidiPtr) -> RtMidiApi;

    #[doc = "! \\brief Immediately send a single message out an open MIDI output port.\n! See \\ref RtMidiOut::sendMessage()."]
    pub fn rtmidi_out_send_message(
        device: RtMidiOutPtr,
        message: *const c_uchar,
        length: c_int,
    ) -> c_int;
}
