use awp::Result;
use axum::response::{IntoResponse, Response};
use human_bytes::human_bytes;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn cpu_flags() -> String {
  let cpuid = raw_cpuid::CpuId::new();

  let mut features = Vec::with_capacity(80);
  cpuid.get_feature_info().map(|finfo| {
    if finfo.has_sse3() {
      features.push("sse3")
    }
    if finfo.has_pclmulqdq() {
      features.push("pclmulqdq")
    }
    if finfo.has_ds_area() {
      features.push("ds_area")
    }
    if finfo.has_monitor_mwait() {
      features.push("monitor_mwait")
    }
    if finfo.has_cpl() {
      features.push("cpl")
    }
    if finfo.has_vmx() {
      features.push("vmx")
    }
    if finfo.has_smx() {
      features.push("smx")
    }
    if finfo.has_eist() {
      features.push("eist")
    }
    if finfo.has_tm2() {
      features.push("tm2")
    }
    if finfo.has_ssse3() {
      features.push("ssse3")
    }
    if finfo.has_cnxtid() {
      features.push("cnxtid")
    }
    if finfo.has_fma() {
      features.push("fma")
    }
    if finfo.has_cmpxchg16b() {
      features.push("cmpxchg16b")
    }
    if finfo.has_pdcm() {
      features.push("pdcm")
    }
    if finfo.has_pcid() {
      features.push("pcid")
    }
    if finfo.has_dca() {
      features.push("dca")
    }
    if finfo.has_sse41() {
      features.push("sse41")
    }
    if finfo.has_sse42() {
      features.push("sse42")
    }
    if finfo.has_x2apic() {
      features.push("x2apic")
    }
    if finfo.has_movbe() {
      features.push("movbe")
    }
    if finfo.has_popcnt() {
      features.push("popcnt")
    }
    if finfo.has_tsc_deadline() {
      features.push("tsc_deadline")
    }
    if finfo.has_aesni() {
      features.push("aesni")
    }
    if finfo.has_xsave() {
      features.push("xsave")
    }
    if finfo.has_oxsave() {
      features.push("oxsave")
    }
    if finfo.has_avx() {
      features.push("avx")
    }
    if finfo.has_f16c() {
      features.push("f16c")
    }
    if finfo.has_rdrand() {
      features.push("rdrand")
    }
    if finfo.has_fpu() {
      features.push("fpu")
    }
    if finfo.has_vme() {
      features.push("vme")
    }
    if finfo.has_de() {
      features.push("de")
    }
    if finfo.has_pse() {
      features.push("pse")
    }
    if finfo.has_tsc() {
      features.push("tsc")
    }
    if finfo.has_msr() {
      features.push("msr")
    }
    if finfo.has_pae() {
      features.push("pae")
    }
    if finfo.has_mce() {
      features.push("mce")
    }
    if finfo.has_cmpxchg8b() {
      features.push("cmpxchg8b")
    }
    if finfo.has_apic() {
      features.push("apic")
    }
    if finfo.has_sysenter_sysexit() {
      features.push("sysenter_sysexit")
    }
    if finfo.has_mtrr() {
      features.push("mtrr")
    }
    if finfo.has_pge() {
      features.push("pge")
    }
    if finfo.has_mca() {
      features.push("mca")
    }
    if finfo.has_cmov() {
      features.push("cmov")
    }
    if finfo.has_pat() {
      features.push("pat")
    }
    if finfo.has_pse36() {
      features.push("pse36")
    }
    if finfo.has_psn() {
      features.push("psn")
    }
    if finfo.has_clflush() {
      features.push("clflush")
    }
    if finfo.has_ds() {
      features.push("ds")
    }
    if finfo.has_acpi() {
      features.push("acpi")
    }
    if finfo.has_mmx() {
      features.push("mmx")
    }
    if finfo.has_fxsave_fxstor() {
      features.push("fxsave_fxstor")
    }
    if finfo.has_sse() {
      features.push("sse")
    }
    if finfo.has_sse2() {
      features.push("sse2")
    }
    if finfo.has_ss() {
      features.push("ss")
    }
    if finfo.has_htt() {
      features.push("htt")
    }
    if finfo.has_tm() {
      features.push("tm")
    }
    if finfo.has_pbe() {
      features.push("pbe")
    }
  });

  cpuid.get_extended_feature_info().map(|finfo| {
    if finfo.has_bmi1() {
      features.push("bmi1")
    }
    if finfo.has_hle() {
      features.push("hle")
    }
    if finfo.has_avx2() {
      features.push("avx2")
    }
    if finfo.has_fdp() {
      features.push("fdp")
    }
    if finfo.has_smep() {
      features.push("smep")
    }
    if finfo.has_bmi2() {
      features.push("bmi2")
    }
    if finfo.has_rep_movsb_stosb() {
      features.push("rep_movsb_stosb")
    }
    if finfo.has_invpcid() {
      features.push("invpcid")
    }
    if finfo.has_rtm() {
      features.push("rtm")
    }
    if finfo.has_rdtm() {
      features.push("rdtm")
    }
    if finfo.has_fpu_cs_ds_deprecated() {
      features.push("fpu_cs_ds_deprecated")
    }
    if finfo.has_mpx() {
      features.push("mpx")
    }
    if finfo.has_rdta() {
      features.push("rdta")
    }
    if finfo.has_rdseed() {
      features.push("rdseed")
    }
    if finfo.has_adx() {
      features.push("adx")
    }
    if finfo.has_smap() {
      features.push("smap")
    }
    if finfo.has_clflushopt() {
      features.push("clflushopt")
    }
    if finfo.has_processor_trace() {
      features.push("processor_trace")
    }
    if finfo.has_sha() {
      features.push("sha")
    }
    if finfo.has_sgx() {
      features.push("sgx")
    }
    if finfo.has_avx512f() {
      features.push("avx512f")
    }
    if finfo.has_avx512dq() {
      features.push("avx512dq")
    }
    if finfo.has_avx512_ifma() {
      features.push("avx512_ifma")
    }
    if finfo.has_avx512pf() {
      features.push("avx512pf")
    }
    if finfo.has_avx512er() {
      features.push("avx512er")
    }
    if finfo.has_avx512cd() {
      features.push("avx512cd")
    }
    if finfo.has_avx512bw() {
      features.push("avx512bw")
    }
    if finfo.has_avx512vl() {
      features.push("avx512vl")
    }
    if finfo.has_clwb() {
      features.push("clwb")
    }
    if finfo.has_prefetchwt1() {
      features.push("prefetchwt1")
    }
    if finfo.has_umip() {
      features.push("umip")
    }
    if finfo.has_pku() {
      features.push("pku")
    }
    if finfo.has_ospke() {
      features.push("ospke")
    }
    if finfo.has_rdpid() {
      features.push("rdpid")
    }
    if finfo.has_sgx_lc() {
      features.push("sgx_lc")
    }
  });

  features.join(" ")
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn cpu_flags() -> String {
  "".into()
}

pub async fn get() -> Result<Response> {
  let s = System::new_with_specifics(
    RefreshKind::new()
      .with_memory()
      .with_cpu(CpuRefreshKind::everything()),
  );
  let cpu = s.global_cpu_info();
  let r = format!(
    "\
sys:
  load: {}
cpu:
  vendor: {}
  brand: {}
  frequency: {}
  physicalCore: {}
  flags: {}
mem:
  total: {}
  used: {}\
  ",
    s.load_average().one,
    cpu.vendor_id(),
    cpu.brand(),
    cpu.frequency(),
    s.physical_core_count().unwrap_or(0),
    cpu_flags(),
    human_bytes(s.total_memory() as f32),
    human_bytes(s.used_memory() as f32),
  );
  Ok(r.into_response())
}
