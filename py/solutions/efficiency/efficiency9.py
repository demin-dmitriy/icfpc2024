from ortools.sat.python import cp_model

def make_model():
    model = cp_model.CpModel()

    num_vals = 9

    v11 = model.new_int_var(0, num_vals - 1, '#11')
    v12 = model.new_int_var(0, num_vals - 1, '#12')
    v13 = model.new_int_var(0, num_vals - 1, '#13')
    v14 = model.new_int_var(0, num_vals - 1, '#14')
    v15 = model.new_int_var(0, num_vals - 1, '#15')
    v16 = model.new_int_var(0, num_vals - 1, '#16')
    v17 = model.new_int_var(0, num_vals - 1, '#17')
    v18 = model.new_int_var(0, num_vals - 1, '#18')
    v19 = model.new_int_var(0, num_vals - 1, '#19')
    v21 = model.new_int_var(0, num_vals - 1, '#21')
    v22 = model.new_int_var(0, num_vals - 1, '#22')
    v23 = model.new_int_var(0, num_vals - 1, '#23')
    v24 = model.new_int_var(0, num_vals - 1, '#24')
    v25 = model.new_int_var(0, num_vals - 1, '#25')
    v26 = model.new_int_var(0, num_vals - 1, '#26')
    v27 = model.new_int_var(0, num_vals - 1, '#27')
    v28 = model.new_int_var(0, num_vals - 1, '#28')
    v29 = model.new_int_var(0, num_vals - 1, '#29')
    v31 = model.new_int_var(0, num_vals - 1, '#31')
    v32 = model.new_int_var(0, num_vals - 1, '#32')
    v33 = model.new_int_var(0, num_vals - 1, '#33')
    v34 = model.new_int_var(0, num_vals - 1, '#34')
    v35 = model.new_int_var(0, num_vals - 1, '#35')
    v36 = model.new_int_var(0, num_vals - 1, '#36')
    v37 = model.new_int_var(0, num_vals - 1, '#37')
    v38 = model.new_int_var(0, num_vals - 1, '#38')
    v39 = model.new_int_var(0, num_vals - 1, '#39')
    v41 = model.new_int_var(0, num_vals - 1, '#41')
    v42 = model.new_int_var(0, num_vals - 1, '#42')
    v43 = model.new_int_var(0, num_vals - 1, '#43')
    v44 = model.new_int_var(0, num_vals - 1, '#44')
    v45 = model.new_int_var(0, num_vals - 1, '#45')
    v46 = model.new_int_var(0, num_vals - 1, '#46')
    v47 = model.new_int_var(0, num_vals - 1, '#47')
    v48 = model.new_int_var(0, num_vals - 1, '#48')
    v49 = model.new_int_var(0, num_vals - 1, '#49')
    v51 = model.new_int_var(0, num_vals - 1, '#51')
    v52 = model.new_int_var(0, num_vals - 1, '#52')
    v53 = model.new_int_var(0, num_vals - 1, '#53')
    v54 = model.new_int_var(0, num_vals - 1, '#54')
    v55 = model.new_int_var(0, num_vals - 1, '#55')
    v56 = model.new_int_var(0, num_vals - 1, '#56')
    v57 = model.new_int_var(0, num_vals - 1, '#57')
    v58 = model.new_int_var(0, num_vals - 1, '#58')
    v59 = model.new_int_var(0, num_vals - 1, '#59')
    v61 = model.new_int_var(0, num_vals - 1, '#61')
    v62 = model.new_int_var(0, num_vals - 1, '#62')
    v63 = model.new_int_var(0, num_vals - 1, '#63')
    v64 = model.new_int_var(0, num_vals - 1, '#64')
    v65 = model.new_int_var(0, num_vals - 1, '#65')
    v66 = model.new_int_var(0, num_vals - 1, '#66')
    v67 = model.new_int_var(0, num_vals - 1, '#67')
    v68 = model.new_int_var(0, num_vals - 1, '#68')
    v69 = model.new_int_var(0, num_vals - 1, '#69')
    v71 = model.new_int_var(0, num_vals - 1, '#71')
    v72 = model.new_int_var(0, num_vals - 1, '#72')
    v73 = model.new_int_var(0, num_vals - 1, '#73')
    v74 = model.new_int_var(0, num_vals - 1, '#74')
    v75 = model.new_int_var(0, num_vals - 1, '#75')
    v76 = model.new_int_var(0, num_vals - 1, '#76')
    v77 = model.new_int_var(0, num_vals - 1, '#77')
    v78 = model.new_int_var(0, num_vals - 1, '#78')
    v79 = model.new_int_var(0, num_vals - 1, '#79')
    v81 = model.new_int_var(0, num_vals - 1, '#81')
    v82 = model.new_int_var(0, num_vals - 1, '#82')
    v83 = model.new_int_var(0, num_vals - 1, '#83')
    v84 = model.new_int_var(0, num_vals - 1, '#84')
    v85 = model.new_int_var(0, num_vals - 1, '#85')
    v86 = model.new_int_var(0, num_vals - 1, '#86')
    v87 = model.new_int_var(0, num_vals - 1, '#87')
    v88 = model.new_int_var(0, num_vals - 1, '#88')
    v89 = model.new_int_var(0, num_vals - 1, '#89')
    v91 = model.new_int_var(0, num_vals - 1, '#91')
    v92 = model.new_int_var(0, num_vals - 1, '#92')
    v93 = model.new_int_var(0, num_vals - 1, '#93')
    v94 = model.new_int_var(0, num_vals - 1, '#94')
    v95 = model.new_int_var(0, num_vals - 1, '#95')
    v96 = model.new_int_var(0, num_vals - 1, '#96')
    v97 = model.new_int_var(0, num_vals - 1, '#97')
    v98 = model.new_int_var(0, num_vals - 1, '#98')
    v99 = model.new_int_var(0, num_vals - 1, '#99')

    variables = [
        v11,
        v12,
        v13,
        v14,
        v15,
        v16,
        v17,
        v18,
        v19,
        v21,
        v22,
        v23,
        v24,
        v25,
        v26,
        v27,
        v28,
        v29,
        v31,
        v32,
        v33,
        v34,
        v35,
        v36,
        v37,
        v38,
        v39,
        v41,
        v42,
        v43,
        v44,
        v45,
        v46,
        v47,
        v48,
        v49,
        v51,
        v52,
        v53,
        v54,
        v55,
        v56,
        v57,
        v58,
        v59,
        v61,
        v62,
        v63,
        v64,
        v65,
        v66,
        v67,
        v68,
        v69,
        v71,
        v72,
        v73,
        v74,
        v75,
        v76,
        v77,
        v78,
        v79,
        v81,
        v82,
        v83,
        v84,
        v85,
        v86,
        v87,
        v88,
        v89,
        v91,
        v92,
        v93,
        v94,
        v95,
        v96,
        v97,
        v98,
        v99,
    ]


    model.add(v11 != v12)
    model.add(v11 != v13)
    model.add(v11 != v14)
    model.add(v11 != v15)
    model.add(v11 != v16)
    model.add(v11 != v17)
    model.add(v11 != v18)
    model.add(v11 != v19)
    model.add(v11 != v21)
    model.add(v11 != v22)
    model.add(v11 != v23)
    model.add(v11 != v31)
    model.add(v11 != v32)
    model.add(v11 != v33)
    model.add(v11 != v41)
    model.add(v11 != v51)
    model.add(v11 != v61)
    model.add(v11 != v71)
    model.add(v11 != v81)
    model.add(v11 != v91)
    model.add(v12 != v13)
    model.add(v12 != v14)
    model.add(v12 != v15)
    model.add(v12 != v16)
    model.add(v12 != v17)
    model.add(v12 != v18)
    model.add(v12 != v19)
    model.add(v12 != v21)
    model.add(v12 != v22)
    model.add(v12 != v23)
    model.add(v12 != v31)
    model.add(v12 != v32)
    model.add(v12 != v33)
    model.add(v12 != v42)
    model.add(v12 != v52)
    model.add(v12 != v62)
    model.add(v12 != v72)
    model.add(v12 != v82)
    model.add(v12 != v92)
    model.add(v13 != v14)
    model.add(v13 != v15)
    model.add(v13 != v16)
    model.add(v13 != v17)
    model.add(v13 != v18)
    model.add(v13 != v19)
    model.add(v13 != v21)
    model.add(v13 != v22)
    model.add(v13 != v23)
    model.add(v13 != v31)
    model.add(v13 != v32)
    model.add(v13 != v33)
    model.add(v13 != v43)
    model.add(v13 != v53)
    model.add(v13 != v63)
    model.add(v13 != v73)
    model.add(v13 != v83)
    model.add(v13 != v93)
    model.add(v14 != v15)
    model.add(v14 != v16)
    model.add(v14 != v17)
    model.add(v14 != v18)
    model.add(v14 != v19)
    model.add(v14 != v24)
    model.add(v14 != v25)
    model.add(v14 != v26)
    model.add(v14 != v34)
    model.add(v14 != v35)
    model.add(v14 != v36)
    model.add(v14 != v44)
    model.add(v14 != v54)
    model.add(v14 != v64)
    model.add(v14 != v74)
    model.add(v14 != v84)
    model.add(v14 != v94)
    model.add(v15 != v16)
    model.add(v15 != v17)
    model.add(v15 != v18)
    model.add(v15 != v19)
    model.add(v15 != v24)
    model.add(v15 != v25)
    model.add(v15 != v26)
    model.add(v15 != v34)
    model.add(v15 != v35)
    model.add(v15 != v36)
    model.add(v15 != v45)
    model.add(v15 != v55)
    model.add(v15 != v65)
    model.add(v15 != v75)
    model.add(v15 != v85)
    model.add(v15 != v95)
    model.add(v16 != v17)
    model.add(v16 != v18)
    model.add(v16 != v19)
    model.add(v16 != v24)
    model.add(v16 != v25)
    model.add(v16 != v26)
    model.add(v16 != v34)
    model.add(v16 != v35)
    model.add(v16 != v36)
    model.add(v16 != v46)
    model.add(v16 != v56)
    model.add(v16 != v66)
    model.add(v16 != v76)
    model.add(v16 != v86)
    model.add(v16 != v96)
    model.add(v17 != v18)
    model.add(v17 != v19)
    model.add(v17 != v27)
    model.add(v17 != v28)
    model.add(v17 != v29)
    model.add(v17 != v37)
    model.add(v17 != v38)
    model.add(v17 != v39)
    model.add(v17 != v47)
    model.add(v17 != v57)
    model.add(v17 != v67)
    model.add(v17 != v77)
    model.add(v17 != v87)
    model.add(v17 != v97)
    model.add(v18 != v19)
    model.add(v18 != v27)
    model.add(v18 != v28)
    model.add(v18 != v29)
    model.add(v18 != v37)
    model.add(v18 != v38)
    model.add(v18 != v39)
    model.add(v18 != v48)
    model.add(v18 != v58)
    model.add(v18 != v68)
    model.add(v18 != v78)
    model.add(v18 != v88)
    model.add(v18 != v98)
    model.add(v19 != v27)
    model.add(v19 != v28)
    model.add(v19 != v29)
    model.add(v19 != v37)
    model.add(v19 != v38)
    model.add(v19 != v39)
    model.add(v19 != v49)
    model.add(v19 != v59)
    model.add(v19 != v69)
    model.add(v19 != v79)
    model.add(v19 != v89)
    model.add(v19 != v99)
    model.add(v21 != v22)
    model.add(v21 != v23)
    model.add(v21 != v24)
    model.add(v21 != v25)
    model.add(v21 != v26)
    model.add(v21 != v27)
    model.add(v21 != v28)
    model.add(v21 != v29)
    model.add(v21 != v31)
    model.add(v21 != v32)
    model.add(v21 != v33)
    model.add(v21 != v41)
    model.add(v21 != v51)
    model.add(v21 != v61)
    model.add(v21 != v71)
    model.add(v21 != v81)
    model.add(v21 != v91)
    model.add(v22 != v23)
    model.add(v22 != v24)
    model.add(v22 != v25)
    model.add(v22 != v26)
    model.add(v22 != v27)
    model.add(v22 != v28)
    model.add(v22 != v29)
    model.add(v22 != v31)
    model.add(v22 != v32)
    model.add(v22 != v33)
    model.add(v22 != v42)
    model.add(v22 != v52)
    model.add(v22 != v62)
    model.add(v22 != v72)
    model.add(v22 != v82)
    model.add(v22 != v92)
    model.add(v23 != v24)
    model.add(v23 != v25)
    model.add(v23 != v26)
    model.add(v23 != v27)
    model.add(v23 != v28)
    model.add(v23 != v29)
    model.add(v23 != v31)
    model.add(v23 != v32)
    model.add(v23 != v33)
    model.add(v23 != v43)
    model.add(v23 != v53)
    model.add(v23 != v63)
    model.add(v23 != v73)
    model.add(v23 != v83)
    model.add(v23 != v93)
    model.add(v24 != v25)
    model.add(v24 != v26)
    model.add(v24 != v27)
    model.add(v24 != v28)
    model.add(v24 != v29)
    model.add(v24 != v34)
    model.add(v24 != v35)
    model.add(v24 != v36)
    model.add(v24 != v44)
    model.add(v24 != v54)
    model.add(v24 != v64)
    model.add(v24 != v74)
    model.add(v24 != v84)
    model.add(v24 != v94)
    model.add(v25 != v26)
    model.add(v25 != v27)
    model.add(v25 != v28)
    model.add(v25 != v29)
    model.add(v25 != v34)
    model.add(v25 != v35)
    model.add(v25 != v36)
    model.add(v25 != v45)
    model.add(v25 != v55)
    model.add(v25 != v65)
    model.add(v25 != v75)
    model.add(v25 != v85)
    model.add(v25 != v95)
    model.add(v26 != v27)
    model.add(v26 != v28)
    model.add(v26 != v29)
    model.add(v26 != v34)
    model.add(v26 != v35)
    model.add(v26 != v36)
    model.add(v26 != v46)
    model.add(v26 != v56)
    model.add(v26 != v66)
    model.add(v26 != v76)
    model.add(v26 != v86)
    model.add(v26 != v96)
    model.add(v27 != v28)
    model.add(v27 != v29)
    model.add(v27 != v37)
    model.add(v27 != v38)
    model.add(v27 != v39)
    model.add(v27 != v47)
    model.add(v27 != v57)
    model.add(v27 != v67)
    model.add(v27 != v77)
    model.add(v27 != v87)
    model.add(v27 != v97)
    model.add(v28 != v29)
    model.add(v28 != v37)
    model.add(v28 != v38)
    model.add(v28 != v39)
    model.add(v28 != v48)
    model.add(v28 != v58)
    model.add(v28 != v68)
    model.add(v28 != v78)
    model.add(v28 != v88)
    model.add(v28 != v98)
    model.add(v29 != v37)
    model.add(v29 != v38)
    model.add(v29 != v39)
    model.add(v29 != v49)
    model.add(v29 != v59)
    model.add(v29 != v69)
    model.add(v29 != v79)
    model.add(v29 != v89)
    model.add(v29 != v99)
    model.add(v31 != v32)
    model.add(v31 != v33)
    model.add(v31 != v34)
    model.add(v31 != v35)
    model.add(v31 != v36)
    model.add(v31 != v37)
    model.add(v31 != v38)
    model.add(v31 != v39)
    model.add(v31 != v41)
    model.add(v31 != v51)
    model.add(v31 != v61)
    model.add(v31 != v71)
    model.add(v31 != v81)
    model.add(v31 != v91)
    model.add(v32 != v33)
    model.add(v32 != v34)
    model.add(v32 != v35)
    model.add(v32 != v36)
    model.add(v32 != v37)
    model.add(v32 != v38)
    model.add(v32 != v39)
    model.add(v32 != v42)
    model.add(v32 != v52)
    model.add(v32 != v62)
    model.add(v32 != v72)
    model.add(v32 != v82)
    model.add(v32 != v92)
    model.add(v33 != v34)
    model.add(v33 != v35)
    model.add(v33 != v36)
    model.add(v33 != v37)
    model.add(v33 != v38)
    model.add(v33 != v39)
    model.add(v33 != v43)
    model.add(v33 != v53)
    model.add(v33 != v63)
    model.add(v33 != v73)
    model.add(v33 != v83)
    model.add(v33 != v93)
    model.add(v34 != v35)
    model.add(v34 != v36)
    model.add(v34 != v37)
    model.add(v34 != v38)
    model.add(v34 != v39)
    model.add(v34 != v44)
    model.add(v34 != v54)
    model.add(v34 != v64)
    model.add(v34 != v74)
    model.add(v34 != v84)
    model.add(v34 != v94)
    model.add(v35 != v36)
    model.add(v35 != v37)
    model.add(v35 != v38)
    model.add(v35 != v39)
    model.add(v35 != v45)
    model.add(v35 != v55)
    model.add(v35 != v65)
    model.add(v35 != v75)
    model.add(v35 != v85)
    model.add(v35 != v95)
    model.add(v36 != v37)
    model.add(v36 != v38)
    model.add(v36 != v39)
    model.add(v36 != v46)
    model.add(v36 != v56)
    model.add(v36 != v66)
    model.add(v36 != v76)
    model.add(v36 != v86)
    model.add(v36 != v96)
    model.add(v37 != v38)
    model.add(v37 != v39)
    model.add(v37 != v47)
    model.add(v37 != v57)
    model.add(v37 != v67)
    model.add(v37 != v77)
    model.add(v37 != v87)
    model.add(v37 != v97)
    model.add(v38 != v39)
    model.add(v38 != v48)
    model.add(v38 != v58)
    model.add(v38 != v68)
    model.add(v38 != v78)
    model.add(v38 != v88)
    model.add(v38 != v98)
    model.add(v39 != v49)
    model.add(v39 != v59)
    model.add(v39 != v69)
    model.add(v39 != v79)
    model.add(v39 != v89)
    model.add(v39 != v99)
    model.add(v41 != v42)
    model.add(v41 != v43)
    model.add(v41 != v44)
    model.add(v41 != v45)
    model.add(v41 != v46)
    model.add(v41 != v47)
    model.add(v41 != v48)
    model.add(v41 != v49)
    model.add(v41 != v51)
    model.add(v41 != v52)
    model.add(v41 != v53)
    model.add(v41 != v61)
    model.add(v41 != v62)
    model.add(v41 != v63)
    model.add(v41 != v71)
    model.add(v41 != v81)
    model.add(v41 != v91)
    model.add(v42 != v43)
    model.add(v42 != v44)
    model.add(v42 != v45)
    model.add(v42 != v46)
    model.add(v42 != v47)
    model.add(v42 != v48)
    model.add(v42 != v49)
    model.add(v42 != v51)
    model.add(v42 != v52)
    model.add(v42 != v53)
    model.add(v42 != v61)
    model.add(v42 != v62)
    model.add(v42 != v63)
    model.add(v42 != v72)
    model.add(v42 != v82)
    model.add(v42 != v92)
    model.add(v43 != v44)
    model.add(v43 != v45)
    model.add(v43 != v46)
    model.add(v43 != v47)
    model.add(v43 != v48)
    model.add(v43 != v49)
    model.add(v43 != v51)
    model.add(v43 != v52)
    model.add(v43 != v53)
    model.add(v43 != v61)
    model.add(v43 != v62)
    model.add(v43 != v63)
    model.add(v43 != v73)
    model.add(v43 != v83)
    model.add(v43 != v93)
    model.add(v44 != v45)
    model.add(v44 != v46)
    model.add(v44 != v47)
    model.add(v44 != v48)
    model.add(v44 != v49)
    model.add(v44 != v54)
    model.add(v44 != v55)
    model.add(v44 != v56)
    model.add(v44 != v64)
    model.add(v44 != v65)
    model.add(v44 != v66)
    model.add(v44 != v74)
    model.add(v44 != v84)
    model.add(v44 != v94)
    model.add(v45 != v46)
    model.add(v45 != v47)
    model.add(v45 != v48)
    model.add(v45 != v49)
    model.add(v45 != v54)
    model.add(v45 != v55)
    model.add(v45 != v56)
    model.add(v45 != v64)
    model.add(v45 != v65)
    model.add(v45 != v66)
    model.add(v45 != v75)
    model.add(v45 != v85)
    model.add(v45 != v95)
    model.add(v46 != v47)
    model.add(v46 != v48)
    model.add(v46 != v49)
    model.add(v46 != v54)
    model.add(v46 != v55)
    model.add(v46 != v56)
    model.add(v46 != v64)
    model.add(v46 != v65)
    model.add(v46 != v66)
    model.add(v46 != v76)
    model.add(v46 != v86)
    model.add(v46 != v96)
    model.add(v47 != v48)
    model.add(v47 != v49)
    model.add(v47 != v57)
    model.add(v47 != v58)
    model.add(v47 != v59)
    model.add(v47 != v67)
    model.add(v47 != v68)
    model.add(v47 != v69)
    model.add(v47 != v77)
    model.add(v47 != v87)
    model.add(v47 != v97)
    model.add(v48 != v49)
    model.add(v48 != v57)
    model.add(v48 != v58)
    model.add(v48 != v59)
    model.add(v48 != v67)
    model.add(v48 != v68)
    model.add(v48 != v69)
    model.add(v48 != v78)
    model.add(v48 != v88)
    model.add(v48 != v98)
    model.add(v49 != v57)
    model.add(v49 != v58)
    model.add(v49 != v59)
    model.add(v49 != v67)
    model.add(v49 != v68)
    model.add(v49 != v69)
    model.add(v49 != v79)
    model.add(v49 != v89)
    model.add(v49 != v99)
    model.add(v51 != v52)
    model.add(v51 != v53)
    model.add(v51 != v54)
    model.add(v51 != v55)
    model.add(v51 != v56)
    model.add(v51 != v57)
    model.add(v51 != v58)
    model.add(v51 != v59)
    model.add(v51 != v61)
    model.add(v51 != v62)
    model.add(v51 != v63)
    model.add(v51 != v71)
    model.add(v51 != v81)
    model.add(v51 != v91)
    model.add(v52 != v53)
    model.add(v52 != v54)
    model.add(v52 != v55)
    model.add(v52 != v56)
    model.add(v52 != v57)
    model.add(v52 != v58)
    model.add(v52 != v59)
    model.add(v52 != v61)
    model.add(v52 != v62)
    model.add(v52 != v63)
    model.add(v52 != v72)
    model.add(v52 != v82)
    model.add(v52 != v92)
    model.add(v53 != v54)
    model.add(v53 != v55)
    model.add(v53 != v56)
    model.add(v53 != v57)
    model.add(v53 != v58)
    model.add(v53 != v59)
    model.add(v53 != v61)
    model.add(v53 != v62)
    model.add(v53 != v63)
    model.add(v53 != v73)
    model.add(v53 != v83)
    model.add(v53 != v93)
    model.add(v54 != v55)
    model.add(v54 != v56)
    model.add(v54 != v57)
    model.add(v54 != v58)
    model.add(v54 != v59)
    model.add(v54 != v64)
    model.add(v54 != v65)
    model.add(v54 != v66)
    model.add(v54 != v74)
    model.add(v54 != v84)
    model.add(v54 != v94)
    model.add(v55 != v56)
    model.add(v55 != v57)
    model.add(v55 != v58)
    model.add(v55 != v59)
    model.add(v55 != v64)
    model.add(v55 != v65)
    model.add(v55 != v66)
    model.add(v55 != v75)
    model.add(v55 != v85)
    model.add(v55 != v95)
    model.add(v56 != v57)
    model.add(v56 != v58)
    model.add(v56 != v59)
    model.add(v56 != v64)
    model.add(v56 != v65)
    model.add(v56 != v66)
    model.add(v56 != v76)
    model.add(v56 != v86)
    model.add(v56 != v96)
    model.add(v57 != v58)
    model.add(v57 != v59)
    model.add(v57 != v67)
    model.add(v57 != v68)
    model.add(v57 != v69)
    model.add(v57 != v77)
    model.add(v57 != v87)
    model.add(v57 != v97)
    model.add(v58 != v59)
    model.add(v58 != v67)
    model.add(v58 != v68)
    model.add(v58 != v69)
    model.add(v58 != v78)
    model.add(v58 != v88)
    model.add(v58 != v98)
    model.add(v59 != v67)
    model.add(v59 != v68)
    model.add(v59 != v69)
    model.add(v59 != v79)
    model.add(v59 != v89)
    model.add(v59 != v99)
    model.add(v61 != v62)
    model.add(v61 != v63)
    model.add(v61 != v64)
    model.add(v61 != v65)
    model.add(v61 != v66)
    model.add(v61 != v67)
    model.add(v61 != v68)
    model.add(v61 != v69)
    model.add(v61 != v71)
    model.add(v61 != v81)
    model.add(v61 != v91)
    model.add(v62 != v63)
    model.add(v62 != v64)
    model.add(v62 != v65)
    model.add(v62 != v66)
    model.add(v62 != v67)
    model.add(v62 != v68)
    model.add(v62 != v69)
    model.add(v62 != v72)
    model.add(v62 != v82)
    model.add(v62 != v92)
    model.add(v63 != v64)
    model.add(v63 != v65)
    model.add(v63 != v66)
    model.add(v63 != v67)
    model.add(v63 != v68)
    model.add(v63 != v69)
    model.add(v63 != v73)
    model.add(v63 != v83)
    model.add(v63 != v93)
    model.add(v64 != v65)
    model.add(v64 != v66)
    model.add(v64 != v67)
    model.add(v64 != v68)
    model.add(v64 != v69)
    model.add(v64 != v74)
    model.add(v64 != v84)
    model.add(v64 != v94)
    model.add(v65 != v66)
    model.add(v65 != v67)
    model.add(v65 != v68)
    model.add(v65 != v69)
    model.add(v65 != v75)
    model.add(v65 != v85)
    model.add(v65 != v95)
    model.add(v66 != v67)
    model.add(v66 != v68)
    model.add(v66 != v69)
    model.add(v66 != v76)
    model.add(v66 != v86)
    model.add(v66 != v96)
    model.add(v67 != v68)
    model.add(v67 != v69)
    model.add(v67 != v77)
    model.add(v67 != v87)
    model.add(v67 != v97)
    model.add(v68 != v69)
    model.add(v68 != v78)
    model.add(v68 != v88)
    model.add(v68 != v98)
    model.add(v69 != v79)
    model.add(v69 != v89)
    model.add(v69 != v99)
    model.add(v71 != v72)
    model.add(v71 != v73)
    model.add(v71 != v74)
    model.add(v71 != v75)
    model.add(v71 != v76)
    model.add(v71 != v77)
    model.add(v71 != v78)
    model.add(v71 != v79)
    model.add(v71 != v81)
    model.add(v71 != v82)
    model.add(v71 != v83)
    model.add(v71 != v91)
    model.add(v71 != v92)
    model.add(v71 != v93)
    model.add(v72 != v73)
    model.add(v72 != v74)
    model.add(v72 != v75)
    model.add(v72 != v76)
    model.add(v72 != v77)
    model.add(v72 != v78)
    model.add(v72 != v79)
    model.add(v72 != v81)
    model.add(v72 != v82)
    model.add(v72 != v83)
    model.add(v72 != v91)
    model.add(v72 != v92)
    model.add(v72 != v93)
    model.add(v73 != v74)
    model.add(v73 != v75)
    model.add(v73 != v76)
    model.add(v73 != v77)
    model.add(v73 != v78)
    model.add(v73 != v79)
    model.add(v73 != v81)
    model.add(v73 != v82)
    model.add(v73 != v83)
    model.add(v73 != v91)
    model.add(v73 != v92)
    model.add(v73 != v93)
    model.add(v74 != v75)
    model.add(v74 != v76)
    model.add(v74 != v77)
    model.add(v74 != v78)
    model.add(v74 != v79)
    model.add(v74 != v84)
    model.add(v74 != v85)
    model.add(v74 != v86)
    model.add(v74 != v94)
    model.add(v74 != v95)
    model.add(v74 != v96)
    model.add(v75 != v76)
    model.add(v75 != v77)
    model.add(v75 != v78)
    model.add(v75 != v79)
    model.add(v75 != v84)
    model.add(v75 != v85)
    model.add(v75 != v86)
    model.add(v75 != v94)
    model.add(v75 != v95)
    model.add(v75 != v96)
    model.add(v76 != v77)
    model.add(v76 != v78)
    model.add(v76 != v79)
    model.add(v76 != v84)
    model.add(v76 != v85)
    model.add(v76 != v86)
    model.add(v76 != v94)
    model.add(v76 != v95)
    model.add(v76 != v96)
    model.add(v77 != v78)
    model.add(v77 != v79)
    model.add(v77 != v87)
    model.add(v77 != v88)
    model.add(v77 != v89)
    model.add(v77 != v97)
    model.add(v77 != v98)
    model.add(v77 != v99)
    model.add(v78 != v79)
    model.add(v78 != v87)
    model.add(v78 != v88)
    model.add(v78 != v89)
    model.add(v78 != v97)
    model.add(v78 != v98)
    model.add(v78 != v99)
    model.add(v79 != v87)
    model.add(v79 != v88)
    model.add(v79 != v89)
    model.add(v79 != v97)
    model.add(v79 != v98)
    model.add(v79 != v99)
    model.add(v81 != v82)
    model.add(v81 != v83)
    model.add(v81 != v84)
    model.add(v81 != v85)
    model.add(v81 != v86)
    model.add(v81 != v87)
    model.add(v81 != v88)
    model.add(v81 != v89)
    model.add(v81 != v91)
    model.add(v81 != v92)
    model.add(v81 != v93)
    model.add(v82 != v83)
    model.add(v82 != v84)
    model.add(v82 != v85)
    model.add(v82 != v86)
    model.add(v82 != v87)
    model.add(v82 != v88)
    model.add(v82 != v89)
    model.add(v82 != v91)
    model.add(v82 != v92)
    model.add(v82 != v93)
    model.add(v83 != v84)
    model.add(v83 != v85)
    model.add(v83 != v86)
    model.add(v83 != v87)
    model.add(v83 != v88)
    model.add(v83 != v89)
    model.add(v83 != v91)
    model.add(v83 != v92)
    model.add(v83 != v93)
    model.add(v84 != v85)
    model.add(v84 != v86)
    model.add(v84 != v87)
    model.add(v84 != v88)
    model.add(v84 != v89)
    model.add(v84 != v94)
    model.add(v84 != v95)
    model.add(v84 != v96)
    model.add(v85 != v86)
    model.add(v85 != v87)
    model.add(v85 != v88)
    model.add(v85 != v89)
    model.add(v85 != v94)
    model.add(v85 != v95)
    model.add(v85 != v96)
    model.add(v86 != v87)
    model.add(v86 != v88)
    model.add(v86 != v89)
    model.add(v86 != v94)
    model.add(v86 != v95)
    model.add(v86 != v96)
    model.add(v87 != v88)
    model.add(v87 != v89)
    model.add(v87 != v97)
    model.add(v87 != v98)
    model.add(v87 != v99)
    model.add(v88 != v89)
    model.add(v88 != v97)
    model.add(v88 != v98)
    model.add(v88 != v99)
    model.add(v89 != v97)
    model.add(v89 != v98)
    model.add(v89 != v99)
    model.add(v91 != v92)
    model.add(v91 != v93)
    model.add(v91 != v94)
    model.add(v91 != v95)
    model.add(v91 != v96)
    model.add(v91 != v97)
    model.add(v91 != v98)
    model.add(v91 != v99)
    model.add(v92 != v93)
    model.add(v92 != v94)
    model.add(v92 != v95)
    model.add(v92 != v96)
    model.add(v92 != v97)
    model.add(v92 != v98)
    model.add(v92 != v99)
    model.add(v93 != v94)
    model.add(v93 != v95)
    model.add(v93 != v96)
    model.add(v93 != v97)
    model.add(v93 != v98)
    model.add(v93 != v99)
    model.add(v94 != v95)
    model.add(v94 != v96)
    model.add(v94 != v97)
    model.add(v94 != v98)
    model.add(v94 != v99)
    model.add(v95 != v96)
    model.add(v95 != v97)
    model.add(v95 != v98)
    model.add(v95 != v99)
    model.add(v96 != v97)
    model.add(v96 != v98)
    model.add(v96 != v99)
    model.add(v97 != v98)
    model.add(v97 != v99)
    model.add(v98 != v99)

    return model, variables


solver = cp_model.CpSolver()
additional_constraints = []
model = None
variables = None

def recreate_model():
    global model
    global variables

    model, variables = make_model()
    for v, c in zip(variables, additional_constraints):
        model.add(v <= c)

recreate_model()

for i in range(len(variables)):
    for bound in range(0, 9):
        model.add(variables[i] <= bound)
        status = solver.solve(model)
        if status != cp_model.OPTIMAL:
            recreate_model()
            continue
        else:
            print(f'{variables[i]} <= {bound}')
            additional_constraints.append(bound)
            break
    else:
        assert False

assert len(additional_constraints) == len(variables)
print(additional_constraints)


values = [solver.value(v) for v in variables]
r = sum(v * 9 ** i for i, v in  enumerate(values[::-1]))

print(''.join([str(v + 1) for v in values]))

print(r)
