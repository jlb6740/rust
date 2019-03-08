use crate::spec::TargetResult;

pub fn target() -> TargetResult {
    let mut base = super::i686_unknown_linux_gnu::target()?;
    base.data_layout = "e-m:e-p:32:32-i64:32-f64:32-f128:32-n8:16:32-a:0:32-S32".to_string();
    base.llvm_target = "i586-unknown-elfiamcu".to_string();
    base.options.cpu = "pentium".to_string();
    Ok(base)
}