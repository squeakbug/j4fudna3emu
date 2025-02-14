# j4fcdna3emu

*We have oneAPI for CPU and oneTBB for other*

* Я ожидал, что к февралю AMD представить [UDNA](https://www.tomshardware.com/pc-components/cpus/amd-announces-unified-udna-gpu-architecture-bringing-rdna-and-cdna-together-to-take-on-nvidias-cuda-ecosystem), микроархитектуру, набор команд и железо (MI350), но что-то пошло не так...

Functional simulator of "AMD Instinct MI300(X)/MI325(X)" (CDNA 3 architecture).

## Goals

Завести XLA

## How to compile programs for CDNA3



## Inspirations

* [CNDA3 hardware overview](https://rocm.docs.amd.com/en/latest/conceptual/gpu-arch/mi300.html)
* [gem5-based clock-accurate simulator for CUDA](https://github.com/gpgpu-sim/gpgpu-sim_distribution)
* [gem5-based AMD APU CAS](https://old.gem5.org/wiki/images/1/19/AMD_gem5_APU_simulator_isca_2018_gem5_wiki.pdf)
* [RDNA3 functional simulator by geohot](https://github.com/geohot/7900xtx)
* [RDNA3 functional simulator for testing tinygrad kernels](https://github.com/Qazalin/remu)
* [GCN3 functional simulator](https://github.com/sarchlab/mgpusim)

https://dl.acm.org/doi/pdf/10.1145/3307650.3322230

## Specs

* [RDNA3.5 ISA](https://www.amd.com/content/dam/amd/en/documents/radeon-tech-docs/instruction-set-architectures/rdna35_instruction_set_architecture.pdf)
* [CDNA3 ISA](https://www.amd.com/content/dam/amd/en/documents/instinct-tech-docs/instruction-set-architectures/amd-instinct-mi300-cdna3-instruction-set-architecture.pdf)

* [AMD Low level compute API - HIP](https://github.com/ROCm/HIP)
* [AMD only High level compute API](https://github.com/ROCm/ROCm?tab=readme-ov-file)
