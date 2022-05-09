; ModuleID = 'probe0.7dfa98f6-cgu.0'
source_filename = "probe0.7dfa98f6-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

@__llvm_profile_runtime = external global i32

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #0 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
