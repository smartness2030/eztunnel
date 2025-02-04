# eZtunnel 🐝

**eZtunnel** is a transparent eBPF-based offloading for service mesh communication designed to mitigate networking overheads in intra-node service mesh communication. The method employs an in-kernel method to bypass costly data paths, while maintaining full support for modern service mesh architectures, like Istio Ambient Mesh, and sidecar-based approaches.

> 🏷️ **Citation**<br>
> If our work assists in your research, kindly cite our paper as follows:
> ```
> @inproceedings{ASimas24:eZtunnel,
>   author={Simas, Arthur J and Rodriguez Cesen, Fabricio E and Rothenberg, Christian Esteve},
>   booktitle={2024 IEEE 13th International Conference on Cloud Networking (CloudNet)}, 
>   title={eZtunnel: Leveraging eBPF to Transparently Offload Service Mesh Data Plane Networking}, 
>   year={2024},
>   pages={1-5},
>   doi={10.1109/CloudNet62863.2024.10815862}
> }
> ```

# Organization

This repository houses the scripts, code and guidance needed to replicate the experiments presented in our papers.

- [eZtunnel Code](./code): Source code for building and running eBPF programs and userspace applications
- [Analysis & Graphs](./graphs): Jupyter Notebooks for data analysis and visualization
- [Workloads & Logs](./workloads): Various workloads and their respective logs generated during the experiments
- [Environment Setup](./setup): Scripts and instructions for setting up the environment for running the experiments
