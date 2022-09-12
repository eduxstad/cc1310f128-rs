# [doc = "Register `UARTCLKGR` reader"] pub struct R (crate :: R < UARTCLKGR_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < UARTCLKGR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < UARTCLKGR_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < UARTCLKGR_SPEC >) -> Self { R (reader) } } # [doc = "Register `UARTCLKGR` writer"] pub struct W (crate :: W < UARTCLKGR_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < UARTCLKGR_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < UARTCLKGR_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < UARTCLKGR_SPEC >) -> Self { W (writer) } } # [doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"] pub type CLK_EN_R = crate :: BitReader < bool > ; # [doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"] pub type CLK_EN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , UARTCLKGR_SPEC , bool , O > ; # [doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] pub type RESERVED1_R = crate :: FieldReader < u32 , u32 > ; # [doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] pub type RESERVED1_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , UARTCLKGR_SPEC , u32 , u32 , 31 , O > ; impl R { # [doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"] # [inline (always)] pub fn clk_en (& self) -> CLK_EN_R { CLK_EN_R :: new ((self . bits & 1) != 0) } # [doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] # [inline (always)] pub fn reserved1 (& self) -> RESERVED1_R { RESERVED1_R :: new (((self . bits >> 1) & 0x7fff_ffff) as u32) } } impl W { # [doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"] # [inline (always)] pub fn clk_en (& mut self) -> CLK_EN_W < 0 > { CLK_EN_W :: new (self) } # [doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] # [inline (always)] pub fn reserved1 (& mut self) -> RESERVED1_W < 1 > { RESERVED1_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "UART Clock Gate For Run Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartclkgr](index.html) module"] pub struct UARTCLKGR_SPEC ; impl crate :: RegisterSpec for UARTCLKGR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [uartclkgr::R](R) reader structure"] impl crate :: Readable for UARTCLKGR_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [uartclkgr::W](W) writer structure"] impl crate :: Writable for UARTCLKGR_SPEC { type Writer = W ; } # [doc = "`reset()` method sets UARTCLKGR to value 0"] impl crate :: Resettable for UARTCLKGR_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }