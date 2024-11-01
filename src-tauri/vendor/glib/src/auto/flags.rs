// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{bitflags::bitflags, prelude::*, translate::*};
use std::fmt;

#[cfg(feature = "v2_66")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GFileSetContentsFlags")]
    pub struct FileSetContentsFlags: u32 {
        #[doc(alias = "G_FILE_SET_CONTENTS_NONE")]
        const NONE = ffi::G_FILE_SET_CONTENTS_NONE as _;
        #[doc(alias = "G_FILE_SET_CONTENTS_CONSISTENT")]
        const CONSISTENT = ffi::G_FILE_SET_CONTENTS_CONSISTENT as _;
        #[doc(alias = "G_FILE_SET_CONTENTS_DURABLE")]
        const DURABLE = ffi::G_FILE_SET_CONTENTS_DURABLE as _;
        #[doc(alias = "G_FILE_SET_CONTENTS_ONLY_EXISTING")]
        const ONLY_EXISTING = ffi::G_FILE_SET_CONTENTS_ONLY_EXISTING as _;
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
impl fmt::Display for FileSetContentsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl IntoGlib for FileSetContentsFlags {
    type GlibType = ffi::GFileSetContentsFlags;

    #[inline]
    fn into_glib(self) -> ffi::GFileSetContentsFlags {
        self.bits()
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GFileSetContentsFlags> for FileSetContentsFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GFileSetContentsFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GFileTest")]
    pub(crate) struct FileTest: u32 {
        #[doc(alias = "G_FILE_TEST_IS_REGULAR")]
        const IS_REGULAR = ffi::G_FILE_TEST_IS_REGULAR as _;
        #[doc(alias = "G_FILE_TEST_IS_SYMLINK")]
        const IS_SYMLINK = ffi::G_FILE_TEST_IS_SYMLINK as _;
        #[doc(alias = "G_FILE_TEST_IS_DIR")]
        const IS_DIR = ffi::G_FILE_TEST_IS_DIR as _;
        #[doc(alias = "G_FILE_TEST_IS_EXECUTABLE")]
        const IS_EXECUTABLE = ffi::G_FILE_TEST_IS_EXECUTABLE as _;
        #[doc(alias = "G_FILE_TEST_EXISTS")]
        const EXISTS = ffi::G_FILE_TEST_EXISTS as _;
    }
}

impl fmt::Display for FileTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FileTest {
    type GlibType = ffi::GFileTest;

    #[inline]
    fn into_glib(self) -> ffi::GFileTest {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileTest> for FileTest {
    #[inline]
    unsafe fn from_glib(value: ffi::GFileTest) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GFormatSizeFlags")]
    pub struct FormatSizeFlags: u32 {
        #[doc(alias = "G_FORMAT_SIZE_DEFAULT")]
        const DEFAULT = ffi::G_FORMAT_SIZE_DEFAULT as _;
        #[doc(alias = "G_FORMAT_SIZE_LONG_FORMAT")]
        const LONG_FORMAT = ffi::G_FORMAT_SIZE_LONG_FORMAT as _;
        #[doc(alias = "G_FORMAT_SIZE_IEC_UNITS")]
        const IEC_UNITS = ffi::G_FORMAT_SIZE_IEC_UNITS as _;
        #[doc(alias = "G_FORMAT_SIZE_BITS")]
        const BITS = ffi::G_FORMAT_SIZE_BITS as _;
        #[cfg(feature = "v2_74")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
        #[doc(alias = "G_FORMAT_SIZE_ONLY_VALUE")]
        const ONLY_VALUE = ffi::G_FORMAT_SIZE_ONLY_VALUE as _;
        #[cfg(feature = "v2_74")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
        #[doc(alias = "G_FORMAT_SIZE_ONLY_UNIT")]
        const ONLY_UNIT = ffi::G_FORMAT_SIZE_ONLY_UNIT as _;
    }
}

impl fmt::Display for FormatSizeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for FormatSizeFlags {
    type GlibType = ffi::GFormatSizeFlags;

    #[inline]
    fn into_glib(self) -> ffi::GFormatSizeFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFormatSizeFlags> for FormatSizeFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GFormatSizeFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GIOCondition")]
    pub struct IOCondition: u32 {
        #[doc(alias = "G_IO_IN")]
        const IN = ffi::G_IO_IN as _;
        #[doc(alias = "G_IO_OUT")]
        const OUT = ffi::G_IO_OUT as _;
        #[doc(alias = "G_IO_PRI")]
        const PRI = ffi::G_IO_PRI as _;
        #[doc(alias = "G_IO_ERR")]
        const ERR = ffi::G_IO_ERR as _;
        #[doc(alias = "G_IO_HUP")]
        const HUP = ffi::G_IO_HUP as _;
        #[doc(alias = "G_IO_NVAL")]
        const NVAL = ffi::G_IO_NVAL as _;
    }
}

impl fmt::Display for IOCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for IOCondition {
    type GlibType = ffi::GIOCondition;

    #[inline]
    fn into_glib(self) -> ffi::GIOCondition {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GIOCondition> for IOCondition {
    #[inline]
    unsafe fn from_glib(value: ffi::GIOCondition) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl StaticType for IOCondition {
    #[inline]
    #[doc(alias = "g_io_condition_get_type")]
    fn static_type() -> crate::Type {
        unsafe { from_glib(ffi::g_io_condition_get_type()) }
    }
}

impl crate::HasParamSpec for IOCondition {
    type ParamSpec = crate::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> crate::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl crate::value::ValueType for IOCondition {
    type Type = Self;
}

unsafe impl<'a> crate::value::FromValue<'a> for IOCondition {
    type Checker = crate::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a crate::Value) -> Self {
        from_glib(crate::gobject_ffi::g_value_get_flags(
            value.to_glib_none().0,
        ))
    }
}

impl ToValue for IOCondition {
    #[inline]
    fn to_value(&self) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            crate::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> crate::Type {
        Self::static_type()
    }
}

impl From<IOCondition> for crate::Value {
    #[inline]
    fn from(v: IOCondition) -> Self {
        ToValue::to_value(&v)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GKeyFileFlags")]
    pub struct KeyFileFlags: u32 {
        #[doc(alias = "G_KEY_FILE_NONE")]
        const NONE = ffi::G_KEY_FILE_NONE as _;
        #[doc(alias = "G_KEY_FILE_KEEP_COMMENTS")]
        const KEEP_COMMENTS = ffi::G_KEY_FILE_KEEP_COMMENTS as _;
        #[doc(alias = "G_KEY_FILE_KEEP_TRANSLATIONS")]
        const KEEP_TRANSLATIONS = ffi::G_KEY_FILE_KEEP_TRANSLATIONS as _;
    }
}

impl fmt::Display for KeyFileFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for KeyFileFlags {
    type GlibType = ffi::GKeyFileFlags;

    #[inline]
    fn into_glib(self) -> ffi::GKeyFileFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GKeyFileFlags> for KeyFileFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GKeyFileFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GLogLevelFlags")]
    pub struct LogLevelFlags: u32 {
        #[doc(alias = "G_LOG_FLAG_RECURSION")]
        const FLAG_RECURSION = ffi::G_LOG_FLAG_RECURSION as _;
        #[doc(alias = "G_LOG_FLAG_FATAL")]
        const FLAG_FATAL = ffi::G_LOG_FLAG_FATAL as _;
        #[doc(alias = "G_LOG_LEVEL_ERROR")]
        const LEVEL_ERROR = ffi::G_LOG_LEVEL_ERROR as _;
        #[doc(alias = "G_LOG_LEVEL_CRITICAL")]
        const LEVEL_CRITICAL = ffi::G_LOG_LEVEL_CRITICAL as _;
        #[doc(alias = "G_LOG_LEVEL_WARNING")]
        const LEVEL_WARNING = ffi::G_LOG_LEVEL_WARNING as _;
        #[doc(alias = "G_LOG_LEVEL_MESSAGE")]
        const LEVEL_MESSAGE = ffi::G_LOG_LEVEL_MESSAGE as _;
        #[doc(alias = "G_LOG_LEVEL_INFO")]
        const LEVEL_INFO = ffi::G_LOG_LEVEL_INFO as _;
        #[doc(alias = "G_LOG_LEVEL_DEBUG")]
        const LEVEL_DEBUG = ffi::G_LOG_LEVEL_DEBUG as _;
        #[doc(alias = "G_LOG_LEVEL_MASK")]
        const LEVEL_MASK = ffi::G_LOG_LEVEL_MASK as _;
    }
}

impl fmt::Display for LogLevelFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for LogLevelFlags {
    type GlibType = ffi::GLogLevelFlags;

    #[inline]
    fn into_glib(self) -> ffi::GLogLevelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GLogLevelFlags> for LogLevelFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GLogLevelFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v2_72")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GMainContextFlags")]
    pub struct MainContextFlags: u32 {
        #[doc(alias = "G_MAIN_CONTEXT_FLAGS_NONE")]
        const NONE = ffi::G_MAIN_CONTEXT_FLAGS_NONE as _;
        #[doc(alias = "G_MAIN_CONTEXT_FLAGS_OWNERLESS_POLLING")]
        const OWNERLESS_POLLING = ffi::G_MAIN_CONTEXT_FLAGS_OWNERLESS_POLLING as _;
    }
}

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
impl fmt::Display for MainContextFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
#[doc(hidden)]
impl IntoGlib for MainContextFlags {
    type GlibType = ffi::GMainContextFlags;

    #[inline]
    fn into_glib(self) -> ffi::GMainContextFlags {
        self.bits()
    }
}

#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
#[doc(hidden)]
impl FromGlib<ffi::GMainContextFlags> for MainContextFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GMainContextFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GOptionFlags")]
    pub struct OptionFlags: u32 {
        #[doc(alias = "G_OPTION_FLAG_NONE")]
        const NONE = ffi::G_OPTION_FLAG_NONE as _;
        #[doc(alias = "G_OPTION_FLAG_HIDDEN")]
        const HIDDEN = ffi::G_OPTION_FLAG_HIDDEN as _;
        #[doc(alias = "G_OPTION_FLAG_IN_MAIN")]
        const IN_MAIN = ffi::G_OPTION_FLAG_IN_MAIN as _;
        #[doc(alias = "G_OPTION_FLAG_REVERSE")]
        const REVERSE = ffi::G_OPTION_FLAG_REVERSE as _;
        #[doc(alias = "G_OPTION_FLAG_NO_ARG")]
        const NO_ARG = ffi::G_OPTION_FLAG_NO_ARG as _;
        #[doc(alias = "G_OPTION_FLAG_FILENAME")]
        const FILENAME = ffi::G_OPTION_FLAG_FILENAME as _;
        #[doc(alias = "G_OPTION_FLAG_OPTIONAL_ARG")]
        const OPTIONAL_ARG = ffi::G_OPTION_FLAG_OPTIONAL_ARG as _;
        #[doc(alias = "G_OPTION_FLAG_NOALIAS")]
        const NOALIAS = ffi::G_OPTION_FLAG_NOALIAS as _;
    }
}

impl fmt::Display for OptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for OptionFlags {
    type GlibType = ffi::GOptionFlags;

    #[inline]
    fn into_glib(self) -> ffi::GOptionFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GOptionFlags> for OptionFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GOptionFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GSpawnFlags")]
    pub struct SpawnFlags: u32 {
        #[doc(alias = "G_SPAWN_DEFAULT")]
        const DEFAULT = ffi::G_SPAWN_DEFAULT as _;
        #[doc(alias = "G_SPAWN_LEAVE_DESCRIPTORS_OPEN")]
        const LEAVE_DESCRIPTORS_OPEN = ffi::G_SPAWN_LEAVE_DESCRIPTORS_OPEN as _;
        #[doc(alias = "G_SPAWN_DO_NOT_REAP_CHILD")]
        const DO_NOT_REAP_CHILD = ffi::G_SPAWN_DO_NOT_REAP_CHILD as _;
        #[doc(alias = "G_SPAWN_SEARCH_PATH")]
        const SEARCH_PATH = ffi::G_SPAWN_SEARCH_PATH as _;
        #[doc(alias = "G_SPAWN_STDOUT_TO_DEV_NULL")]
        const STDOUT_TO_DEV_NULL = ffi::G_SPAWN_STDOUT_TO_DEV_NULL as _;
        #[doc(alias = "G_SPAWN_STDERR_TO_DEV_NULL")]
        const STDERR_TO_DEV_NULL = ffi::G_SPAWN_STDERR_TO_DEV_NULL as _;
        #[doc(alias = "G_SPAWN_CHILD_INHERITS_STDIN")]
        const CHILD_INHERITS_STDIN = ffi::G_SPAWN_CHILD_INHERITS_STDIN as _;
        #[doc(alias = "G_SPAWN_FILE_AND_ARGV_ZERO")]
        const FILE_AND_ARGV_ZERO = ffi::G_SPAWN_FILE_AND_ARGV_ZERO as _;
        #[doc(alias = "G_SPAWN_SEARCH_PATH_FROM_ENVP")]
        const SEARCH_PATH_FROM_ENVP = ffi::G_SPAWN_SEARCH_PATH_FROM_ENVP as _;
        #[doc(alias = "G_SPAWN_CLOEXEC_PIPES")]
        const CLOEXEC_PIPES = ffi::G_SPAWN_CLOEXEC_PIPES as _;
        #[cfg(feature = "v2_74")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
        #[doc(alias = "G_SPAWN_CHILD_INHERITS_STDOUT")]
        const CHILD_INHERITS_STDOUT = ffi::G_SPAWN_CHILD_INHERITS_STDOUT as _;
        #[cfg(feature = "v2_74")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
        #[doc(alias = "G_SPAWN_CHILD_INHERITS_STDERR")]
        const CHILD_INHERITS_STDERR = ffi::G_SPAWN_CHILD_INHERITS_STDERR as _;
        #[cfg(feature = "v2_74")]
        #[cfg_attr(docsrs, doc(cfg(feature = "v2_74")))]
        #[doc(alias = "G_SPAWN_STDIN_FROM_DEV_NULL")]
        const STDIN_FROM_DEV_NULL = ffi::G_SPAWN_STDIN_FROM_DEV_NULL as _;
    }
}

impl fmt::Display for SpawnFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[doc(hidden)]
impl IntoGlib for SpawnFlags {
    type GlibType = ffi::GSpawnFlags;

    #[inline]
    fn into_glib(self) -> ffi::GSpawnFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSpawnFlags> for SpawnFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GSpawnFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v2_66")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GUriFlags")]
    pub struct UriFlags: u32 {
        #[doc(alias = "G_URI_FLAGS_NONE")]
        const NONE = ffi::G_URI_FLAGS_NONE as _;
        #[doc(alias = "G_URI_FLAGS_PARSE_RELAXED")]
        const PARSE_RELAXED = ffi::G_URI_FLAGS_PARSE_RELAXED as _;
        #[doc(alias = "G_URI_FLAGS_HAS_PASSWORD")]
        const HAS_PASSWORD = ffi::G_URI_FLAGS_HAS_PASSWORD as _;
        #[doc(alias = "G_URI_FLAGS_HAS_AUTH_PARAMS")]
        const HAS_AUTH_PARAMS = ffi::G_URI_FLAGS_HAS_AUTH_PARAMS as _;
        #[doc(alias = "G_URI_FLAGS_ENCODED")]
        const ENCODED = ffi::G_URI_FLAGS_ENCODED as _;
        #[doc(alias = "G_URI_FLAGS_NON_DNS")]
        const NON_DNS = ffi::G_URI_FLAGS_NON_DNS as _;
        #[doc(alias = "G_URI_FLAGS_ENCODED_QUERY")]
        const ENCODED_QUERY = ffi::G_URI_FLAGS_ENCODED_QUERY as _;
        #[doc(alias = "G_URI_FLAGS_ENCODED_PATH")]
        const ENCODED_PATH = ffi::G_URI_FLAGS_ENCODED_PATH as _;
        #[doc(alias = "G_URI_FLAGS_ENCODED_FRAGMENT")]
        const ENCODED_FRAGMENT = ffi::G_URI_FLAGS_ENCODED_FRAGMENT as _;
        #[doc(alias = "G_URI_FLAGS_SCHEME_NORMALIZE")]
        const SCHEME_NORMALIZE = ffi::G_URI_FLAGS_SCHEME_NORMALIZE as _;
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl IntoGlib for UriFlags {
    type GlibType = ffi::GUriFlags;

    #[inline]
    fn into_glib(self) -> ffi::GUriFlags {
        self.bits()
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriFlags> for UriFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GUriFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v2_66")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GUriHideFlags")]
    pub struct UriHideFlags: u32 {
        #[doc(alias = "G_URI_HIDE_NONE")]
        const NONE = ffi::G_URI_HIDE_NONE as _;
        #[doc(alias = "G_URI_HIDE_USERINFO")]
        const USERINFO = ffi::G_URI_HIDE_USERINFO as _;
        #[doc(alias = "G_URI_HIDE_PASSWORD")]
        const PASSWORD = ffi::G_URI_HIDE_PASSWORD as _;
        #[doc(alias = "G_URI_HIDE_AUTH_PARAMS")]
        const AUTH_PARAMS = ffi::G_URI_HIDE_AUTH_PARAMS as _;
        #[doc(alias = "G_URI_HIDE_QUERY")]
        const QUERY = ffi::G_URI_HIDE_QUERY as _;
        #[doc(alias = "G_URI_HIDE_FRAGMENT")]
        const FRAGMENT = ffi::G_URI_HIDE_FRAGMENT as _;
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriHideFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl IntoGlib for UriHideFlags {
    type GlibType = ffi::GUriHideFlags;

    #[inline]
    fn into_glib(self) -> ffi::GUriHideFlags {
        self.bits()
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriHideFlags> for UriHideFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GUriHideFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v2_66")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GUriParamsFlags")]
    pub struct UriParamsFlags: u32 {
        #[doc(alias = "G_URI_PARAMS_NONE")]
        const NONE = ffi::G_URI_PARAMS_NONE as _;
        #[doc(alias = "G_URI_PARAMS_CASE_INSENSITIVE")]
        const CASE_INSENSITIVE = ffi::G_URI_PARAMS_CASE_INSENSITIVE as _;
        #[doc(alias = "G_URI_PARAMS_WWW_FORM")]
        const WWW_FORM = ffi::G_URI_PARAMS_WWW_FORM as _;
        #[doc(alias = "G_URI_PARAMS_PARSE_RELAXED")]
        const PARSE_RELAXED = ffi::G_URI_PARAMS_PARSE_RELAXED as _;
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
impl fmt::Display for UriParamsFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl IntoGlib for UriParamsFlags {
    type GlibType = ffi::GUriParamsFlags;

    #[inline]
    fn into_glib(self) -> ffi::GUriParamsFlags {
        self.bits()
    }
}

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
#[doc(hidden)]
impl FromGlib<ffi::GUriParamsFlags> for UriParamsFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GUriParamsFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}