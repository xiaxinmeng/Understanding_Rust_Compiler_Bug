LLVM
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.26.28806"

define i32 @main() unnamed_addr {
entry:
  %proc_heap = tail call i8* @GetProcessHeap()
  %vec_ptr0 = tail call i8* @HeapAlloc(i8* %proc_heap, i32 0, i64 8)
  br label %loop_hdr
loop_hdr:                                         ; preds = %loop_write, %entry
  %vec_ptr1 = phi i8* [ %vec_ptr0, %entry ], [ %vec_ptr2, %loop_write ]
  %vec_cap1 = phi i64 [ 8, %entry ], [ %new_vec_cap1, %loop_write ]
  %vec_len1 = phi i64 [ 0, %entry ], [ %new_vec_len1, %loop_write ]
  %val = phi i8 [ 0, %entry ], [ %1, %loop_write ]
  %0 = icmp eq i8 %val, 255
  %1 = add nuw i8 %val, 1
  %2 = sub i64 %vec_cap1, %vec_len1
  %3 = icmp ult i64 %2, 2
  br i1 %3, label %check_realloc, label %pre_loop_write
check_realloc:                   ; preds = %loop_hdr
  %len_plus_two_checked = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %vec_len1, i64 2)
  %checked_res = extractvalue { i64, i1 } %len_plus_two_checked, 1
  br i1 %checked_res, label %exit, label %realloc
realloc:                         ; preds = %check_realloc
  %len_plus_two = extractvalue { i64, i1 } %len_plus_two_checked, 0
  %cap_times_two = shl i64 %vec_cap1, 1
  %4 = icmp ugt i64 %cap_times_two, %len_plus_two
  %new_cap = select i1 %4, i64 %cap_times_two, i64 %len_plus_two
  %new_vec_ptr = tail call i8* @HeapReAlloc(i8* %proc_heap, i32 0, i8* %vec_ptr1, i64 %new_cap)
  br label %loop_write
pre_loop_write:                                             ; preds = %loop_hdr
  %new_vec_len2 = add i64 %vec_len1, 2
  br label %loop_write
loop_write:                                       ; preds = %realloc, %pre_loop_write
  %new_vec_len1 = phi i64 [ %new_vec_len2, %pre_loop_write ], [ %len_plus_two, %realloc ]
  %vec_ptr2 = phi i8* [ %vec_ptr1, %pre_loop_write ], [ %new_vec_ptr, %realloc ]
  %new_vec_cap1 = phi i64 [ %vec_cap1, %pre_loop_write ], [ %new_cap, %realloc ]
  %vec_store_slot1 = getelementptr inbounds i8, i8* %vec_ptr2, i64 %vec_len1
  store i8 %val, i8* %vec_store_slot1, align 1
  %vec_store_slot2 = getelementptr inbounds i8, i8* %vec_store_slot1, i64 1
  store i8 %val, i8* %vec_store_slot2, align 1
  br i1 %0, label %exit, label %loop_hdr
exit:                                             ; preds = %loop_write, %check_realloc
  %exit_code = phi i32 [ 0, %loop_write ], [ 1, %check_realloc ]
  ret i32 %exit_code
}
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64)
declare i8* @GetProcessHeap() unnamed_addr
declare i8* @HeapAlloc(i8*, i32, i64) unnamed_addr
declare i8* @HeapReAlloc(i8*, i32, i8*, i64) unnamed_addr
