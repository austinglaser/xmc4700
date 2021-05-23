#[doc = "Register `PRESENT_STATE` reader"]
pub struct R(crate::R<PRESENT_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESENT_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRESENT_STATE_SPEC>> for R {
    fn from(reader: crate::R<PRESENT_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DAT_7_4_PIN_LEVEL` reader - Line Signal Level"]
pub struct DAT_7_4_PIN_LEVEL_R(crate::FieldReader<u8, u8>);
impl DAT_7_4_PIN_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_7_4_PIN_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_7_4_PIN_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_LINE_LEVEL` reader - CMD Line Signal Level"]
pub struct CMD_LINE_LEVEL_R(crate::FieldReader<bool, bool>);
impl CMD_LINE_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMD_LINE_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMD_LINE_LEVEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_3_0_PIN_LEVEL` reader - Line Signal Level"]
pub struct DAT_3_0_PIN_LEVEL_R(crate::FieldReader<u8, u8>);
impl DAT_3_0_PIN_LEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAT_3_0_PIN_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAT_3_0_PIN_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Protect Switch Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_PROTECT_PIN_LEVEL_A {
    #[doc = "0: Write protected (SDWP = 1)"]
    VALUE1 = 0,
    #[doc = "1: Write enabled (SDWP = 0)"]
    VALUE2 = 1,
}
impl From<WRITE_PROTECT_PIN_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_PROTECT_PIN_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_PROTECT_PIN_LEVEL` reader - Write Protect Switch Pin Level"]
pub struct WRITE_PROTECT_PIN_LEVEL_R(crate::FieldReader<bool, WRITE_PROTECT_PIN_LEVEL_A>);
impl WRITE_PROTECT_PIN_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_PROTECT_PIN_LEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_PROTECT_PIN_LEVEL_A {
        match self.bits {
            false => WRITE_PROTECT_PIN_LEVEL_A::VALUE1,
            true => WRITE_PROTECT_PIN_LEVEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WRITE_PROTECT_PIN_LEVEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WRITE_PROTECT_PIN_LEVEL_A::VALUE2
    }
}
impl core::ops::Deref for WRITE_PROTECT_PIN_LEVEL_R {
    type Target = crate::FieldReader<bool, WRITE_PROTECT_PIN_LEVEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Detect Pin Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_DETECT_PIN_LEVEL_A {
    #[doc = "0: No Card present (SDCD = 1)"]
    VALUE1 = 0,
    #[doc = "1: Card present (SDCD = 0)"]
    VALUE2 = 1,
}
impl From<CARD_DETECT_PIN_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_DETECT_PIN_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_DETECT_PIN_LEVEL` reader - Card Detect Pin Level"]
pub struct CARD_DETECT_PIN_LEVEL_R(crate::FieldReader<bool, CARD_DETECT_PIN_LEVEL_A>);
impl CARD_DETECT_PIN_LEVEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_DETECT_PIN_LEVEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_DETECT_PIN_LEVEL_A {
        match self.bits {
            false => CARD_DETECT_PIN_LEVEL_A::VALUE1,
            true => CARD_DETECT_PIN_LEVEL_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CARD_DETECT_PIN_LEVEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_DETECT_PIN_LEVEL_A::VALUE2
    }
}
impl core::ops::Deref for CARD_DETECT_PIN_LEVEL_R {
    type Target = crate::FieldReader<bool, CARD_DETECT_PIN_LEVEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card State Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_STATE_STABLE_A {
    #[doc = "0: Reset of Debouncing"]
    VALUE1 = 0,
    #[doc = "1: No Card or Inserted"]
    VALUE2 = 1,
}
impl From<CARD_STATE_STABLE_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_STATE_STABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_STATE_STABLE` reader - Card State Stable"]
pub struct CARD_STATE_STABLE_R(crate::FieldReader<bool, CARD_STATE_STABLE_A>);
impl CARD_STATE_STABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_STATE_STABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_STATE_STABLE_A {
        match self.bits {
            false => CARD_STATE_STABLE_A::VALUE1,
            true => CARD_STATE_STABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CARD_STATE_STABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_STATE_STABLE_A::VALUE2
    }
}
impl core::ops::Deref for CARD_STATE_STABLE_R {
    type Target = crate::FieldReader<bool, CARD_STATE_STABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CARD_INSERTED_A {
    #[doc = "0: Reset or Debouncing or No Card"]
    VALUE1 = 0,
    #[doc = "1: Card Inserted"]
    VALUE2 = 1,
}
impl From<CARD_INSERTED_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INSERTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARD_INSERTED` reader - Card Inserted"]
pub struct CARD_INSERTED_R(crate::FieldReader<bool, CARD_INSERTED_A>);
impl CARD_INSERTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CARD_INSERTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INSERTED_A {
        match self.bits {
            false => CARD_INSERTED_A::VALUE1,
            true => CARD_INSERTED_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == CARD_INSERTED_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == CARD_INSERTED_A::VALUE2
    }
}
impl core::ops::Deref for CARD_INSERTED_R {
    type Target = crate::FieldReader<bool, CARD_INSERTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_READ_ENABLE_A {
    #[doc = "0: Read Disable"]
    VALUE1 = 0,
    #[doc = "1: Read Enable."]
    VALUE2 = 1,
}
impl From<BUFFER_READ_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_READ_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFER_READ_ENABLE` reader - Buffer Read Enable"]
pub struct BUFFER_READ_ENABLE_R(crate::FieldReader<bool, BUFFER_READ_ENABLE_A>);
impl BUFFER_READ_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFFER_READ_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_READ_ENABLE_A {
        match self.bits {
            false => BUFFER_READ_ENABLE_A::VALUE1,
            true => BUFFER_READ_ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUFFER_READ_ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFFER_READ_ENABLE_A::VALUE2
    }
}
impl core::ops::Deref for BUFFER_READ_ENABLE_R {
    type Target = crate::FieldReader<bool, BUFFER_READ_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUFFER_WRITE_ENABLE_A {
    #[doc = "0: Write Disable"]
    VALUE1 = 0,
    #[doc = "1: Write Enable."]
    VALUE2 = 1,
}
impl From<BUFFER_WRITE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BUFFER_WRITE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFFER_WRITE_ENABLE` reader - Buffer Write Enable"]
pub struct BUFFER_WRITE_ENABLE_R(crate::FieldReader<bool, BUFFER_WRITE_ENABLE_A>);
impl BUFFER_WRITE_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFFER_WRITE_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUFFER_WRITE_ENABLE_A {
        match self.bits {
            false => BUFFER_WRITE_ENABLE_A::VALUE1,
            true => BUFFER_WRITE_ENABLE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == BUFFER_WRITE_ENABLE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == BUFFER_WRITE_ENABLE_A::VALUE2
    }
}
impl core::ops::Deref for BUFFER_WRITE_ENABLE_R {
    type Target = crate::FieldReader<bool, BUFFER_WRITE_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_TRANSFER_ACTIVE_A {
    #[doc = "0: No valid data"]
    VALUE1 = 0,
    #[doc = "1: Transferring data"]
    VALUE2 = 1,
}
impl From<READ_TRANSFER_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: READ_TRANSFER_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_TRANSFER_ACTIVE` reader - Read Transfer Active"]
pub struct READ_TRANSFER_ACTIVE_R(crate::FieldReader<bool, READ_TRANSFER_ACTIVE_A>);
impl READ_TRANSFER_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_TRANSFER_ACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_TRANSFER_ACTIVE_A {
        match self.bits {
            false => READ_TRANSFER_ACTIVE_A::VALUE1,
            true => READ_TRANSFER_ACTIVE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == READ_TRANSFER_ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == READ_TRANSFER_ACTIVE_A::VALUE2
    }
}
impl core::ops::Deref for READ_TRANSFER_ACTIVE_R {
    type Target = crate::FieldReader<bool, READ_TRANSFER_ACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_TRANSFER_ACTIVE_A {
    #[doc = "0: No valid data"]
    VALUE1 = 0,
    #[doc = "1: Transferring data"]
    VALUE2 = 1,
}
impl From<WRITE_TRANSFER_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_TRANSFER_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_TRANSFER_ACTIVE` reader - Write Transfer Active"]
pub struct WRITE_TRANSFER_ACTIVE_R(crate::FieldReader<bool, WRITE_TRANSFER_ACTIVE_A>);
impl WRITE_TRANSFER_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_TRANSFER_ACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_TRANSFER_ACTIVE_A {
        match self.bits {
            false => WRITE_TRANSFER_ACTIVE_A::VALUE1,
            true => WRITE_TRANSFER_ACTIVE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == WRITE_TRANSFER_ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == WRITE_TRANSFER_ACTIVE_A::VALUE2
    }
}
impl core::ops::Deref for WRITE_TRANSFER_ACTIVE_R {
    type Target = crate::FieldReader<bool, WRITE_TRANSFER_ACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "DAT Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_LINE_ACTIVE_A {
    #[doc = "0: DAT line inactive"]
    VALUE1 = 0,
    #[doc = "1: DAT line active"]
    VALUE2 = 1,
}
impl From<DAT_LINE_ACTIVE_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_LINE_ACTIVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_LINE_ACTIVE` reader - DAT Line Active"]
pub struct DAT_LINE_ACTIVE_R(crate::FieldReader<bool, DAT_LINE_ACTIVE_A>);
impl DAT_LINE_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAT_LINE_ACTIVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_LINE_ACTIVE_A {
        match self.bits {
            false => DAT_LINE_ACTIVE_A::VALUE1,
            true => DAT_LINE_ACTIVE_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == DAT_LINE_ACTIVE_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == DAT_LINE_ACTIVE_A::VALUE2
    }
}
impl core::ops::Deref for DAT_LINE_ACTIVE_R {
    type Target = crate::FieldReader<bool, DAT_LINE_ACTIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMMAND_INHIBIT_DAT_A {
    #[doc = "0: Can issue command which uses the DAT line"]
    VALUE1 = 0,
    #[doc = "1: Cannot issue command which uses the DAT line"]
    VALUE2 = 1,
}
impl From<COMMAND_INHIBIT_DAT_A> for bool {
    #[inline(always)]
    fn from(variant: COMMAND_INHIBIT_DAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMMAND_INHIBIT_DAT` reader - Command Inhibit (DAT)"]
pub struct COMMAND_INHIBIT_DAT_R(crate::FieldReader<bool, COMMAND_INHIBIT_DAT_A>);
impl COMMAND_INHIBIT_DAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND_INHIBIT_DAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMMAND_INHIBIT_DAT_A {
        match self.bits {
            false => COMMAND_INHIBIT_DAT_A::VALUE1,
            true => COMMAND_INHIBIT_DAT_A::VALUE2,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        **self == COMMAND_INHIBIT_DAT_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        **self == COMMAND_INHIBIT_DAT_A::VALUE2
    }
}
impl core::ops::Deref for COMMAND_INHIBIT_DAT_R {
    type Target = crate::FieldReader<bool, COMMAND_INHIBIT_DAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMAND_INHIBIT_CMD` reader - Command Inhibit (CMD)"]
pub struct COMMAND_INHIBIT_CMD_R(crate::FieldReader<bool, bool>);
impl COMMAND_INHIBIT_CMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMMAND_INHIBIT_CMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMAND_INHIBIT_CMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 25:28 - Line Signal Level"]
    #[inline(always)]
    pub fn dat_7_4_pin_level(&self) -> DAT_7_4_PIN_LEVEL_R {
        DAT_7_4_PIN_LEVEL_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn cmd_line_level(&self) -> CMD_LINE_LEVEL_R {
        CMD_LINE_LEVEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Line Signal Level"]
    #[inline(always)]
    pub fn dat_3_0_pin_level(&self) -> DAT_3_0_PIN_LEVEL_R {
        DAT_3_0_PIN_LEVEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Write Protect Switch Pin Level"]
    #[inline(always)]
    pub fn write_protect_pin_level(&self) -> WRITE_PROTECT_PIN_LEVEL_R {
        WRITE_PROTECT_PIN_LEVEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Card Detect Pin Level"]
    #[inline(always)]
    pub fn card_detect_pin_level(&self) -> CARD_DETECT_PIN_LEVEL_R {
        CARD_DETECT_PIN_LEVEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card State Stable"]
    #[inline(always)]
    pub fn card_state_stable(&self) -> CARD_STATE_STABLE_R {
        CARD_STATE_STABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CARD_INSERTED_R {
        CARD_INSERTED_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn buffer_read_enable(&self) -> BUFFER_READ_ENABLE_R {
        BUFFER_READ_ENABLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn buffer_write_enable(&self) -> BUFFER_WRITE_ENABLE_R {
        BUFFER_WRITE_ENABLE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn read_transfer_active(&self) -> READ_TRANSFER_ACTIVE_R {
        READ_TRANSFER_ACTIVE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn write_transfer_active(&self) -> WRITE_TRANSFER_ACTIVE_R {
        WRITE_TRANSFER_ACTIVE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DAT Line Active"]
    #[inline(always)]
    pub fn dat_line_active(&self) -> DAT_LINE_ACTIVE_R {
        DAT_LINE_ACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn command_inhibit_dat(&self) -> COMMAND_INHIBIT_DAT_R {
        COMMAND_INHIBIT_DAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn command_inhibit_cmd(&self) -> COMMAND_INHIBIT_CMD_R {
        COMMAND_INHIBIT_CMD_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Present State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [present_state](index.html) module"]
pub struct PRESENT_STATE_SPEC;
impl crate::RegisterSpec for PRESENT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [present_state::R](R) reader structure"]
impl crate::Readable for PRESENT_STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRESENT_STATE to value 0"]
impl crate::Resettable for PRESENT_STATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
