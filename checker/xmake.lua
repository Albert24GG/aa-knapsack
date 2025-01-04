set_languages("c17")
add_rules("mode.release", "mode.debug")

target("check")
    set_kind("binary") 
    add_files("check.c")

target("knapsack_dp")
    set_kind("binary") 
    add_files("knapsack_dp.c")
