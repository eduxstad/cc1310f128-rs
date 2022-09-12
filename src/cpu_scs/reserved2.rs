# [doc = "Register `RESERVED2` reader"] pub struct R (crate :: R < RESERVED2_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < RESERVED2_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < RESERVED2_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < RESERVED2_SPEC >) -> Self { R (reader) } } # [doc = "Register `RESERVED2` writer"] pub struct W (crate :: W < RESERVED2_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < RESERVED2_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < RESERVED2_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < RESERVED2_SPEC >) -> Self { W (writer) } } # [doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] pub type RESERVED0_R = crate :: FieldReader < u32 , u32 > ; # [doc = "Field `RESERVED0` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] pub type RESERVED0_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , RESERVED2_SPEC , u32 , u32 , 32 , O > ; impl R { # [doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] # [inline (always)] pub fn reserved0 (& self) -> RESERVED0_R { RESERVED0_R :: new (self . bits) } } impl W { # [doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."] # [inline (always)] pub fn reserved0 (& mut self) -> RESERVED0_W < 0 > { RESERVED0_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved2](index.html) module"] pub struct RESERVED2_SPEC ; impl crate :: RegisterSpec for RESERVED2_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [reserved2::R](R) reader structure"] impl crate :: Readable for RESERVED2_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [reserved2::W](W) writer structure"] impl crate :: Writable for RESERVED2_SPEC { type Writer = W ; } # [doc = "`reset()` method sets RESERVED2 to value 0"] impl crate :: Resettable for RESERVED2_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }