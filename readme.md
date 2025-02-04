# Gowin EDA wrapper-toolchain

#### 0. Create `.gowin` file contains absolute path to the Gowin EDA installation dir
```
/home/limpix/gowin
```
and also gitignore it

#### 1. Create `Gowiners.toml` file in root of the project
and fill it as follows
```toml
name = "key-led"
version = 5

# Device options
[device]
family = "GW5AST-138B"
part = "GW5AST-LV138FPG676AC1/I0"

# Hdl options
[hdl]
# Use System Verilog 2017
standard = "sysv2017"
# Top module name
top = "top"
# Dirs to include
include = ["include"]

# Project files
[files]
cst = "phy.cst"
verilog = [
	"rtl/top.sv"
]

# Place And Route config
[pnr]
place_mode = 0
route_mode = 1
replicate = false

# Bitstream generation config
[bitstream]
compress = true

# Programmer config
[programmer]
# Bitstream file
fs = "impl/pnr/project.fs"
frequency = "2.5Mhz"
cable = "Gowin USB Cable(FT2CH)"

# SRAM Preset
preset.sram = { op = "2" } # SRAM Program
# External flash burn preset
preset.burn = { op = "53" } # exFlash Erase,Program 5A

```

#### 2. Run implementation via cli
```bash
gowiners impl
```
Example output:
```
Run implementation
  Evaluating project
  Run Gowin EDA syn and pnr tasks
*** GOWIN Tcl Command Line Console  *** 
current device: GW5AST-138B  GW5AST-LV138FPG676AC1/I0
add new file: "/home/limpix/gowiners/example/phy.cst"
add new file: "/home/limpix/gowiners/example/rtl/top.sv"
GowinSynthesis start
Running parser ...
Analyzing Verilog file '/home/limpix/gowiners/example/rtl/top.sv'
Compiling module 'top'("/home/limpix/gowiners/example/rtl/top.sv":1)
NOTE  (EX0101) : Current top module is "top"
[5%] Running netlist conversion ...
Running device independent optimization ...
[10%] Optimizing Phase 0 completed
[15%] Optimizing Phase 1 completed
[25%] Optimizing Phase 2 completed
Running inference ...
[30%] Inferring Phase 0 completed
[40%] Inferring Phase 1 completed
[50%] Inferring Phase 2 completed
[55%] Inferring Phase 3 completed
Running technical mapping ...
[60%] Tech-Mapping Phase 0 completed
[65%] Tech-Mapping Phase 1 completed
[75%] Tech-Mapping Phase 2 completed
[80%] Tech-Mapping Phase 3 completed
[90%] Tech-Mapping Phase 4 completed
[95%] Generate netlist file "/home/limpix/gowiners/example/impl/gwsynthesis/project.vg" completed
[100%] Generate report file "/home/limpix/gowiners/example/impl/gwsynthesis/project_syn.rpt.html" completed
GowinSynthesis finish
Reading netlist file: "/home/limpix/gowiners/example/impl/gwsynthesis/project.vg"
Parsing netlist file "/home/limpix/gowiners/example/impl/gwsynthesis/project.vg" completed
Processing netlist completed
Reading constraint file: "/home/limpix/gowiners/example/phy.cst"
Physical Constraint parsed completed
Running placement......
[10%] Placement Phase 0 completed
[20%] Placement Phase 1 completed
[30%] Placement Phase 2 completed
[50%] Placement Phase 3 completed
Running routing......
[60%] Routing Phase 0 completed
[70%] Routing Phase 1 completed
[80%] Routing Phase 2 completed
[90%] Routing Phase 3 completed
Running timing analysis......
[95%] Timing analysis completed
Placement and routing completed
Bitstream generation in progress......
Bitstream generation completed
Running power analysis......
[100%] Power analysis completed
Generate file "/home/limpix/gowiners/example/impl/pnr/project.power.html" completed
Generate file "/home/limpix/gowiners/example/impl/pnr/project.pin.html" completed
Generate file "/home/limpix/gowiners/example/impl/pnr/project.rpt.html" completed
Generate file "/home/limpix/gowiners/example/impl/pnr/project.rpt.txt" completed
Generate file "/home/limpix/gowiners/example/impl/pnr/project.tr.html" completed
Mon Jan  6 14:14:16 2025

Completed
```

#### 3. Flash bitstream
Drive your design in real life
```bash
# Where `sram` is programmer preset defined earlier in Gowiners.toml
gowiners flash sram
```
