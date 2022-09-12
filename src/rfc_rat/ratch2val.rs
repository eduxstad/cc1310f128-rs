# [doc = "Register `RATCH2VAL` reader"] pub struct R (crate :: R < RATCH2VAL_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < RATCH2VAL_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < RATCH2VAL_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < RATCH2VAL_SPEC >) -> Self { R (reader) } } # [doc = "Register `RATCH2VAL` writer"] pub struct W (crate :: W < RATCH2VAL_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < RATCH2VAL_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < RATCH2VAL_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < RATCH2VAL_SPEC >) -> Self { W (writer) } } # [doc = "Field `VAL` reader - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."] pub type VAL_R = crate :: FieldReader < u32 , u32 > ; # [doc = "Field `VAL` writer - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."] pub type VAL_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , RATCH2VAL_SPEC , u32 , u32 , 32 , O > ; impl R { # [doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."] # [inline (always)] pub fn val (& self) -> VAL_R { VAL_R :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."] # [inline (always)] pub fn val (& mut self) -> VAL_W < 0 > { VAL_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Timer Channel 2 Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratch2val](index.html) module"] pub struct RATCH2VAL_SPEC ; impl crate :: RegisterSpec for RATCH2VAL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [ratch2val::R](R) reader structure"] impl crate :: Readable for RATCH2VAL_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [ratch2val::W](W) writer structure"] impl crate :: Writable for RATCH2VAL_SPEC { type Writer = W ; } # [doc = "`reset()` method sets RATCH2VAL to value 0"] impl crate :: Resettable for RATCH2VAL_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }