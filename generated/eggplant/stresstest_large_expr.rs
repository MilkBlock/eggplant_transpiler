// Generated Eggplant Rust Code
// Source files referenced in comments below
use eggplant::{{prelude::*, tx_rx_vt_pr}};

// Source: examples/stresstest_large_expr.egg:5
// Pattern variables for rule matching
// Variables: __var__i, __var___
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var___: Expr,
}

// Source: examples/stresstest_large_expr.egg:10
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:12
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:14
// Pattern variables for rule matching
// Variables: __var__stop
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__stop: Expr,
}

// Source: examples/stresstest_large_expr.egg:18
// Pattern variables for rule matching
// Variables: __var__vec
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__vec: Expr,
}

// Source: examples/stresstest_large_expr.egg:26
// Pattern variables for rule matching
// Variables: __var__other, __var__self, __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__other: Expr,
    __var__self: Expr,
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:29
// Pattern variables for rule matching
// Variables: __var__self, __var__other
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__self: Expr,
    __var__other: Expr,
}

// Source: examples/stresstest_large_expr.egg:33
// Pattern variables for rule matching
// Variables: __var__i, __var__acc, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__acc: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:36
// Pattern variables for rule matching
// Variables: __var__self, __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__self: Expr,
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:41
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:49
// Pattern variables for rule matching
// Variables: __var__a
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__a: Expr,
}

// Source: examples/stresstest_large_expr.egg:53
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:55
// Pattern variables for rule matching
// Variables: __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:63
// Pattern variables for rule matching
// Variables: __var__arr
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__arr: Expr,
}

// Source: examples/stresstest_large_expr.egg:65
// Pattern variables for rule matching
// Variables: __var__x, __var__vs
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__vs: Expr,
}

// Source: examples/stresstest_large_expr.egg:68
// Pattern variables for rule matching
// Variables: __var__x, __var__vs
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__vs: Expr,
}

// Source: examples/stresstest_large_expr.egg:69
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:70
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:71
// Pattern variables for rule matching
// Variables: __var__x, __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:72
// Pattern variables for rule matching
// Variables: __var__x, __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:74
// Pattern variables for rule matching
// Variables: __var__x, __var__shape
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__shape: Expr,
}

// Source: examples/stresstest_large_expr.egg:75
// Pattern variables for rule matching
// Variables: __var__x, __var__shape
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__shape: Expr,
}

// Source: examples/stresstest_large_expr.egg:76
// Pattern variables for rule matching
// Variables: __var__x, __var__shape, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__shape: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:78
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
}

// Source: examples/stresstest_large_expr.egg:79
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
}

// Source: examples/stresstest_large_expr.egg:80
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:84
// Pattern variables for rule matching
// Variables: __var__x, __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:90
// Pattern variables for rule matching
// Variables: __var__vs
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__vs: Expr,
}

// Source: examples/stresstest_large_expr.egg:93
// Pattern variables for rule matching
// Variables: __var__vs
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__vs: Expr,
}

// Source: examples/stresstest_large_expr.egg:94
// Pattern variables for rule matching
// Variables: __var__vs, __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__vs: Expr,
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:96
// Pattern variables for rule matching
// Variables: __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:97
// Pattern variables for rule matching
// Variables: __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:98
// Pattern variables for rule matching
// Variables: __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:102
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:107
// Pattern variables for rule matching
// Variables: __var__x, __var__y, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__y: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:110
// Pattern variables for rule matching
// Variables: __var__x, __var__y, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__y: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:111
// Pattern variables for rule matching
// Variables: __var__v, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:114
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype, __var__idx
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
    __var__idx: Expr,
}

// Source: examples/stresstest_large_expr.egg:124
// Pattern variables for rule matching
// Variables: __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:127
// Pattern variables for rule matching
// Variables: __var__x, __var__full_matrices
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__full_matrices: Expr,
}

// Source: examples/stresstest_large_expr.egg:129
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:130
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:133
// Pattern variables for rule matching
// Variables: __var__f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:134
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:135
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:136
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
}

// Source: examples/stresstest_large_expr.egg:137
// Pattern variables for rule matching
// Variables: __var__x, __var__dtype
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__dtype: Expr,
}

// Source: examples/stresstest_large_expr.egg:139
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:144
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:145
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:146
// Pattern variables for rule matching
// Variables: __var__x, __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:152
// Pattern variables for rule matching
// Variables: __var__a, __var__d, __var__ob
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__a: Expr,
    __var__d: Expr,
    __var__ob: Expr,
}

// Source: examples/stresstest_large_expr.egg:155
// Pattern variables for rule matching
// Variables: __var__a
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__a: Expr,
}

// Source: examples/stresstest_large_expr.egg:158
// Pattern variables for rule matching
// Variables: __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:159
// Pattern variables for rule matching
// Variables: __var__n
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__n: Expr,
}

// Source: examples/stresstest_large_expr.egg:160
// Pattern variables for rule matching
// Variables: __var__ti, __var__ti2
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__ti: Expr,
    __var__ti2: Expr,
}

// Source: examples/stresstest_large_expr.egg:161
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:163
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:164
// Pattern variables for rule matching
// Variables: __var__f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:166
// Pattern variables for rule matching
// Variables: __var__f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:168
// Pattern variables for rule matching
// Variables: __var__fi1, __var__fi2
#[eggplant::pat_vars]
struct rule_168Pat {
    __var__fi1: Expr,
    __var__fi2: Expr,
}

// Source: examples/stresstest_large_expr.egg:171
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:174
// Pattern variables for rule matching
// Variables: __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:175
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:176
// Pattern variables for rule matching
// Variables: __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:177
// Pattern variables for rule matching
// Variables: __var__ti, __var__ti2
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__ti: Expr,
    __var__ti2: Expr,
}

// Source: examples/stresstest_large_expr.egg:178
// Pattern variables for rule matching
// Variables: __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:179
// Pattern variables for rule matching
// Variables: __var__v, __var__ti
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__v: Expr,
    __var__ti: Expr,
}

// Source: examples/stresstest_large_expr.egg:180
// Pattern variables for rule matching
// Variables: __var__ti, __var__ti2, __var__v
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__ti: Expr,
    __var__ti2: Expr,
    __var__v: Expr,
}

// Source: examples/stresstest_large_expr.egg:182
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:183
// Pattern variables for rule matching
// Variables: __var__f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:185
// Pattern variables for rule matching
// Variables: __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:187
// Pattern variables for rule matching
// Variables: __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:189
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:190
// Pattern variables for rule matching
// Variables: __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:195
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:196
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:198
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:199
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:201
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:202
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:203
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:204
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:205
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:206
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:207
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:208
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:209
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:210
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:211
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:213
// Pattern variables for rule matching
// Variables: __var__d
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__d: Expr,
}

// Source: examples/stresstest_large_expr.egg:215
// Pattern variables for rule matching
// Variables: __var__d
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__d: Expr,
}

// Source: examples/stresstest_large_expr.egg:216
// Pattern variables for rule matching
// Variables: __var__k1
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__k1: Expr,
}

// Source: examples/stresstest_large_expr.egg:218
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:219
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:220
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:221
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:222
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:223
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:224
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:225
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:226
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:227
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:228
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:229
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:230
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:231
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:232
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:233
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:234
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:235
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:236
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:237
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:238
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:239
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:240
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:241
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:242
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:244
// Pattern variables for rule matching
// Variables: __var__idx_fn, __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__idx_fn: Expr,
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:245
// Pattern variables for rule matching
// Variables: __var__vs, __var__k
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__vs: Expr,
    __var__k: Expr,
}

// Source: examples/stresstest_large_expr.egg:246
// Pattern variables for rule matching
// Variables: __var__i, __var__idx_fn
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__idx_fn: Expr,
}

// Source: examples/stresstest_large_expr.egg:247
// Pattern variables for rule matching
// Variables: __var__i, __var__idx_fn, __var__i2
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__idx_fn: Expr,
    __var__i2: Expr,
}

// Source: examples/stresstest_large_expr.egg:248
// Pattern variables for rule matching
// Variables: __var__idx_fn, __var__i, __var__f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__idx_fn: Expr,
    __var__i: Expr,
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:249
// Pattern variables for rule matching
// Variables: __var__idx_fn, __var__b, __var__bool_f
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__idx_fn: Expr,
    __var__b: Expr,
    __var__bool_f: Expr,
}

// Source: examples/stresstest_large_expr.egg:251
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:252
// Pattern variables for rule matching
// Variables: __var__f
#[eggplant::pat_vars]
struct rule_252Pat {
    __var__f: Expr,
}

// Source: examples/stresstest_large_expr.egg:256
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:258
// Pattern variables for rule matching
// Variables: __var__f, __var__f2
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
    __var__f2: Expr,
}

// Source: examples/stresstest_large_expr.egg:261
// Pattern variables for rule matching
// Variables: __var__f, __var__f2
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__f: Expr,
    __var__f2: Expr,
}

// Source: examples/stresstest_large_expr.egg:263
// Pattern variables for rule matching
// Variables: __var__r, __var__r1
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__r: Expr,
    __var__r1: Expr,
}

// Source: examples/stresstest_large_expr.egg:264
// Pattern variables for rule matching
// Variables: __var__r, __var__r1
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__r: Expr,
    __var__r1: Expr,
}

// Source: examples/stresstest_large_expr.egg:265
// Pattern variables for rule matching
// Variables: __var__r, __var__r1
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__r: Expr,
    __var__r1: Expr,
}

// Source: examples/stresstest_large_expr.egg:266
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:268
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:269
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:271
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:272
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:273
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:275
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:277
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:279
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:281
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:283
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:285
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:287
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:289
// Pattern variables for rule matching
// Variables: __var__i
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
}

// Source: examples/stresstest_large_expr.egg:290
// Pattern variables for rule matching
// Variables: __var__o, __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__o: Expr,
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:291
// Pattern variables for rule matching
// Variables: __var__o, __var__b
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__o: Expr,
    __var__b: Expr,
}

// Source: examples/stresstest_large_expr.egg:294
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:295
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:297
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:298
// Pattern variables for rule matching
// Variables: __var__x
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__x: Expr,
}

// Source: examples/stresstest_large_expr.egg:300
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:301
// Pattern variables for rule matching
// Variables: __var__i, __var__j
#[eggplant::pat_vars]
struct array_api_rulesetPat {
    __var__i: Expr,
    __var__j: Expr,
}

// Source: examples/stresstest_large_expr.egg:303
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

// Source: examples/stresstest_large_expr.egg:304
// Pattern variables for rule matching
// Variables: 
#[eggplant::pat_vars]
struct array_api_rulesetPat {
}

fn main() {
    // Source: examples/stresstest_large_expr.egg:1
    tx_rx_vt_pr!(MyTx, MyPatRec);
    
    // Source: examples/stresstest_large_expr.egg:5
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let __var___ = Expr::query_leaf();
let cast_callable__int__int___int___lambda_i_____i__node1 = cast_Callable__Int__Int___Int___lambda_i_____i_::query(&__var__i, &__var___);
array_api_rulesetPat::new(__var__i, __var___)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:10
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let tupleint_single_node1 = TupleInt_single::query(&__var__i);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = TupleInt___init__::new(pat.int___init___node2, pat.unstable_fn_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:12
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let cast_callable__int___int___lambda_i__i__node1 = cast_Callable__Int___Int___lambda_i__i_::query(&__var__i);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:14
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__stop = Expr::query_leaf();
let tupleint_range_node1 = TupleInt_range::query(&__var__stop);
array_api_rulesetPat::new(__var__stop)
        },
        |ctx, pat| {
            let result = TupleInt___init__::new(pat.__var__stop, pat.unstable_fn_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:18
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__vec = Expr::query_leaf();
let tupleint_from_vec_node1 = TupleInt_from_vec::query(&__var__vec);
array_api_rulesetPat::new(__var__vec)
        },
        |ctx, pat| {
            let result = TupleInt___init__::new(pat.int___init___node2, pat.unstable_fn_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:26
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__other = Expr::query_leaf();
let __var__self = Expr::query_leaf();
let __var__i = Expr::query_leaf();
let cast_callable__tupleint__tupleint__int___int___lambda_other__self__i__int_if__i___self_length____self_i___other_i___self_length______node1 = cast_Callable__TupleInt__TupleInt__Int___Int___lambda_other__self__i__Int_if__i___self_length____self_i___other_i___self_length_____::query(&__var__other, &__var__self, &__var__i);
array_api_rulesetPat::new(__var__other, __var__self, __var__i)
        },
        |ctx, pat| {
            let result = Int_if_::new(pat.int___lt___node2, pat.tupleint___getitem___node3, pat.tupleint___getitem___node4);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:29
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__self = Expr::query_leaf();
let __var__other = Expr::query_leaf();
let tupleint___add___node1 = TupleInt___add__::query(&__var__self, &__var__other);
array_api_rulesetPat::new(__var__self, __var__other)
        },
        |ctx, pat| {
            let result = TupleInt___init__::new(pat.int___add___node2, pat.unstable_fn_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:33
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let __var__acc = Expr::query_leaf();
let __var__j = Expr::query_leaf();
let cast_callable__int__boolean__int___boolean___lambda_i__acc__j__acc____i____j___node1 = cast_Callable__Int__Boolean__Int___Boolean___lambda_i__acc__j__acc____i____j__::query(&__var__i, &__var__acc, &__var__j);
array_api_rulesetPat::new(__var__i, __var__acc, __var__j)
        },
        |ctx, pat| {
            let result = Boolean___or__::new(pat.__var__acc, pat.int___eq___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:36
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__self = Expr::query_leaf();
let __var__i = Expr::query_leaf();
let tupleint_contains_node1 = TupleInt_contains::query(&__var__self, &__var__i);
array_api_rulesetPat::new(__var__self, __var__i)
        },
        |ctx, pat| {
            let result = TupleInt_fold_boolean::new(pat.__var__self, pat.false_node2, pat.unstable_fn_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:41
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let ndarray_size_node1 = NDArray_size::query(&__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = TupleInt_fold::new(pat.ndarray_shape_node2, pat.int___init___node3, pat.unstable_fn_node4);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:49
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__a = Expr::query_leaf();
let unique_values_node1 = unique_values::query(&__var__a);
array_api_rulesetPat::new(__var__a)
        },
        |ctx, pat| {
            let result = NDArray_vector::new(pat.possible_values_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:53
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let value_int_node1 = Value_int::query(&__var__i);
let value_isfinite_node3 = Value_isfinite::query(&value_int_node2);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:55
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__b = Expr::query_leaf();
let value_bool_node1 = Value_bool::query(&__var__b);
let value_isfinite_node3 = Value_isfinite::query(&value_bool_node2);
array_api_rulesetPat::new(__var__b)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:63
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__arr = Expr::query_leaf();
let optionalintortuple_none_node1 = OptionalIntOrTuple_none::query();
let sum_node3 = sum::query(&__var__arr, &optionalintortuple_none_node2);
let isfinite_node5 = isfinite::query(&sum_node4);
array_api_rulesetPat::new(__var__arr)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_bool_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:65
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__vs = Expr::query_leaf();
let assume_value_one_of_node1 = assume_value_one_of::query(&__var__x, &__var__vs);
let ndarray_shape_node3 = NDArray_shape::query(&assume_value_one_of_node2);
array_api_rulesetPat::new(__var__x, __var__vs)
        },
        |ctx, pat| {
            let result = NDArray_shape::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:68
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__vs = Expr::query_leaf();
let assume_value_one_of_node1 = assume_value_one_of::query(&__var__x, &__var__vs);
let ndarray_dtype_node3 = NDArray_dtype::query(&assume_value_one_of_node2);
array_api_rulesetPat::new(__var__x, __var__vs)
        },
        |ctx, pat| {
            let result = NDArray_dtype::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:69
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let assume_isfinite_node1 = assume_isfinite::query(&__var__x);
let ndarray_shape_node3 = NDArray_shape::query(&assume_isfinite_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = NDArray_shape::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:70
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let assume_isfinite_node1 = assume_isfinite::query(&__var__x);
let ndarray_dtype_node3 = NDArray_dtype::query(&assume_isfinite_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = NDArray_dtype::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:71
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let assume_isfinite_node1 = assume_isfinite::query(&__var__x);
let __var__ti = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&assume_isfinite_node2, &__var__ti);
array_api_rulesetPat::new(__var__x, __var__ti)
        },
        |ctx, pat| {
            let result = NDArray_index::new(pat.__var__x, pat.__var__ti);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:72
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let assume_isfinite_node1 = assume_isfinite::query(&__var__x);
let __var__ti = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&assume_isfinite_node2, &__var__ti);
let value_isfinite_node5 = Value_isfinite::query(&ndarray_index_node4);
array_api_rulesetPat::new(__var__x, __var__ti)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:74
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__shape = Expr::query_leaf();
let assume_shape_node1 = assume_shape::query(&__var__x, &__var__shape);
let ndarray_shape_node3 = NDArray_shape::query(&assume_shape_node2);
array_api_rulesetPat::new(__var__x, __var__shape)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__shape);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:75
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__shape = Expr::query_leaf();
let assume_shape_node1 = assume_shape::query(&__var__x, &__var__shape);
let ndarray_dtype_node3 = NDArray_dtype::query(&assume_shape_node2);
array_api_rulesetPat::new(__var__x, __var__shape)
        },
        |ctx, pat| {
            let result = NDArray_dtype::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:76
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__shape = Expr::query_leaf();
let assume_shape_node1 = assume_shape::query(&__var__x, &__var__shape);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&assume_shape_node2, &__var__idx);
array_api_rulesetPat::new(__var__x, __var__shape, __var__idx)
        },
        |ctx, pat| {
            let result = NDArray_index::new(pat.__var__x, pat.__var__idx);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:78
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__dtype = Expr::query_leaf();
let assume_dtype_node1 = assume_dtype::query(&__var__x, &__var__dtype);
let ndarray_dtype_node3 = NDArray_dtype::query(&assume_dtype_node2);
array_api_rulesetPat::new(__var__x, __var__dtype)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__dtype);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:79
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__dtype = Expr::query_leaf();
let assume_dtype_node1 = assume_dtype::query(&__var__x, &__var__dtype);
let ndarray_shape_node3 = NDArray_shape::query(&assume_dtype_node2);
array_api_rulesetPat::new(__var__x, __var__dtype)
        },
        |ctx, pat| {
            let result = NDArray_shape::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:80
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__dtype = Expr::query_leaf();
let assume_dtype_node1 = assume_dtype::query(&__var__x, &__var__dtype);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&assume_dtype_node2, &__var__idx);
array_api_rulesetPat::new(__var__x, __var__dtype, __var__idx)
        },
        |ctx, pat| {
            let result = NDArray_index::new(pat.__var__x, pat.__var__idx);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:84
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__i = Expr::query_leaf();
let indexkey_int_node1 = IndexKey_int::query(&__var__i);
let ndarray___getitem___node3 = NDArray___getitem__::query(&__var__x, &indexkey_int_node2);
array_api_rulesetPat::new(__var__x, __var__i)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.ndarray_index_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:90
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__vs = Expr::query_leaf();
let ndarray_vector_node1 = NDArray_vector::query(&__var__vs);
let ndarray_shape_node3 = NDArray_shape::query(&ndarray_vector_node2);
array_api_rulesetPat::new(__var__vs)
        },
        |ctx, pat| {
            let result = TupleInt_single::new(pat.tuplevalue_length_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:93
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__vs = Expr::query_leaf();
let ndarray_vector_node1 = NDArray_vector::query(&__var__vs);
let ndarray_dtype_node3 = NDArray_dtype::query(&ndarray_vector_node2);
array_api_rulesetPat::new(__var__vs)
        },
        |ctx, pat| {
            let result = Value_dtype::new(pat.tuplevalue___getitem___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:94
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__vs = Expr::query_leaf();
let ndarray_vector_node1 = NDArray_vector::query(&__var__vs);
let __var__ti = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&ndarray_vector_node2, &__var__ti);
array_api_rulesetPat::new(__var__vs, __var__ti)
        },
        |ctx, pat| {
            let result = TupleValue___getitem__::new(pat.__var__vs, pat.tupleint___getitem___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:96
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let ndarray_scalar_node1 = NDArray_scalar::query(&__var__v);
let ndarray_shape_node3 = NDArray_shape::query(&ndarray_scalar_node2);
array_api_rulesetPat::new(__var__v)
        },
        |ctx, pat| {
            let result = TupleInt_EMPTY::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:97
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let ndarray_scalar_node1 = NDArray_scalar::query(&__var__v);
let ndarray_dtype_node3 = NDArray_dtype::query(&ndarray_scalar_node2);
array_api_rulesetPat::new(__var__v)
        },
        |ctx, pat| {
            let result = Value_dtype::new(pat.__var__v);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:98
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let ndarray_scalar_node1 = NDArray_scalar::query(&__var__v);
let tupleint_empty_node2 = TupleInt_EMPTY::query();
let ndarray_index_node5 = NDArray_index::query(&ndarray_scalar_node3, &tupleint_empty_node4);
array_api_rulesetPat::new(__var__v)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__v);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:102
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let any_node1 = any::query(&__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_bool_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:107
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__y = Expr::query_leaf();
let ndarray___lt___node1 = NDArray___lt__::query(&__var__x, &__var__y);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&ndarray___lt___node2, &__var__idx);
array_api_rulesetPat::new(__var__x, __var__y, __var__idx)
        },
        |ctx, pat| {
            let result = Value___lt__::new(pat.ndarray_index_node2, pat.ndarray_index_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:110
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__y = Expr::query_leaf();
let ndarray___truediv___node1 = NDArray___truediv__::query(&__var__x, &__var__y);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&ndarray___truediv___node2, &__var__idx);
array_api_rulesetPat::new(__var__x, __var__y, __var__idx)
        },
        |ctx, pat| {
            let result = Value___truediv__::new(pat.ndarray_index_node2, pat.ndarray_index_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:111
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let ndarray_scalar_node1 = NDArray_scalar::query(&__var__v);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&ndarray_scalar_node2, &__var__idx);
array_api_rulesetPat::new(__var__v, __var__idx)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__v);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:114
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__dtype = Expr::query_leaf();
let astype_node1 = astype::query(&__var__x, &__var__dtype);
let __var__idx = Expr::query_leaf();
let ndarray_index_node3 = NDArray_index::query(&astype_node2, &__var__idx);
array_api_rulesetPat::new(__var__x, __var__dtype, __var__idx)
        },
        |ctx, pat| {
            let result = Value_astype::new(pat.ndarray_index_node2, pat.__var__dtype);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:124
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__b = Expr::query_leaf();
let value_bool_node1 = Value_bool::query(&__var__b);
let possible_values_node3 = possible_values::query(&value_bool_node2);
array_api_rulesetPat::new(__var__b)
        },
        |ctx, pat| {
            let result = TupleValue___init__::new(pat.value_bool_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:127
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__full_matrices = Expr::query_leaf();
let svd_node1 = svd::query(&__var__x, &__var__full_matrices);
let tuplendarray_length_node3 = TupleNDArray_length::query(&svd_node2);
array_api_rulesetPat::new(__var__x, __var__full_matrices)
        },
        |ctx, pat| {
            let result = Int___init__::new(3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:129
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_inverse_node1 = unique_inverse::query(&__var__x);
let tuplendarray_length_node3 = TupleNDArray_length::query(&unique_inverse_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = Int___init__::new(2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:130
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_inverse_node1 = unique_inverse::query(&__var__x);
let int___init___node2 = Int___init__::query(0);
let tuplendarray___getitem___node5 = TupleNDArray___getitem__::query(&unique_inverse_node3, &int___init___node4);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = unique_values::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:133
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let value_float_node1 = Value_float::query(&__var__f);
let ndarray_scalar_node3 = NDArray_scalar::query(&value_float_node2);
let ndarray_abs_node5 = ndarray_abs::query(&ndarray_scalar_node4);
array_api_rulesetPat::new(__var__f)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_float_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:134
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_counts_node1 = unique_counts::query(&__var__x);
let tuplendarray_length_node3 = TupleNDArray_length::query(&unique_counts_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = Int___init__::new(2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:135
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_counts_node1 = unique_counts::query(&__var__x);
let int___init___node2 = Int___init__::query(1);
let tuplendarray___getitem___node5 = TupleNDArray___getitem__::query(&unique_counts_node3, &int___init___node4);
let optionalintortuple_none_node6 = OptionalIntOrTuple_none::query();
let sum_node9 = sum::query(&tuplendarray___getitem___node7, &optionalintortuple_none_node8);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_int_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:136
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_counts_node1 = unique_counts::query(&__var__x);
let int___init___node2 = Int___init__::query(1);
let tuplendarray___getitem___node5 = TupleNDArray___getitem__::query(&unique_counts_node3, &int___init___node4);
let __var__dtype = Expr::query_leaf();
let astype_node7 = astype::query(&tuplendarray___getitem___node6, &__var__dtype);
let optionalintortuple_none_node8 = OptionalIntOrTuple_none::query();
let sum_node11 = sum::query(&astype_node9, &optionalintortuple_none_node10);
array_api_rulesetPat::new(__var__x, __var__dtype)
        },
        |ctx, pat| {
            let result = astype::new(pat.ndarray_scalar_node2, pat.__var__dtype);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:137
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__dtype = Expr::query_leaf();
let astype_node1 = astype::query(&__var__x, &__var__dtype);
let ndarray_dtype_node3 = NDArray_dtype::query(&astype_node2);
array_api_rulesetPat::new(__var__x, __var__dtype)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__dtype);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:139
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let value_int_node3 = Value_int::query(&int___init___node2);
let ndarray_scalar_node5 = NDArray_scalar::query(&value_int_node4);
let dtype_float64_node6 = DType_float64::query();
let astype_node9 = astype::query(&ndarray_scalar_node7, &dtype_float64_node8);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_float_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:144
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let tuplendarray___init___node1 = TupleNDArray___init__::query(&__var__x);
let optionalint_none_node2 = OptionalInt_none::query();
let concat_node5 = concat::query(&tuplendarray___init___node3, &optionalint_none_node4);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__x);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:145
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let unique_values_node1 = unique_values::query(&__var__x);
let unique_values_node3 = unique_values::query(&unique_values_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = unique_values::new(pat.__var__x);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:146
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let __var__v = Expr::query_leaf();
let ndarray_scalar_node1 = NDArray_scalar::query(&__var__v);
let ndarray___truediv___node3 = NDArray___truediv__::query(&__var__x, &ndarray_scalar_node2);
let optionalintortuple_none_node4 = OptionalIntOrTuple_none::query();
let sum_node7 = sum::query(&ndarray___truediv___node5, &optionalintortuple_none_node6);
array_api_rulesetPat::new(__var__x, __var__v)
        },
        |ctx, pat| {
            let result = NDArray___truediv__::new(pat.sum_node2, pat.ndarray_scalar_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:152
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__a = Expr::query_leaf();
let __var__d = Expr::query_leaf();
let __var__ob = Expr::query_leaf();
let optionaldevice_none_node1 = OptionalDevice_none::query();
let asarray_node3 = asarray::query(&__var__a, &__var__d, &__var__ob, &optionaldevice_none_node2);
let ndarray_ndim_node5 = NDArray_ndim::query(&asarray_node4);
array_api_rulesetPat::new(__var__a, __var__d, __var__ob)
        },
        |ctx, pat| {
            let result = NDArray_ndim::new(pat.__var__a);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:155
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__a = Expr::query_leaf();
let optionaldtype_none_node1 = OptionalDType_none::query();
let optionalbool_none_node2 = OptionalBool_none::query();
let optionaldevice_none_node3 = OptionalDevice_none::query();
let asarray_node7 = asarray::query(&__var__a, &optionaldtype_none_node4, &optionalbool_none_node5, &optionaldevice_none_node6);
array_api_rulesetPat::new(__var__a)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__a);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:158
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__ti = Expr::query_leaf();
let tuplendarray_empty_node1 = TupleNDArray_EMPTY::query();
let tuplendarray___add___node3 = TupleNDArray___add__::query(&__var__ti, &tuplendarray_empty_node2);
array_api_rulesetPat::new(__var__ti)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__ti);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:159
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__n = Expr::query_leaf();
let tuplendarray___init___node1 = TupleNDArray___init__::query(&__var__n);
let tuplendarray_length_node3 = TupleNDArray_length::query(&tuplendarray___init___node2);
array_api_rulesetPat::new(__var__n)
        },
        |ctx, pat| {
            let result = Int___init__::new(1);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:160
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__ti = Expr::query_leaf();
let __var__ti2 = Expr::query_leaf();
let tuplendarray___add___node1 = TupleNDArray___add__::query(&__var__ti, &__var__ti2);
let tuplendarray_length_node3 = TupleNDArray_length::query(&tuplendarray___add___node2);
array_api_rulesetPat::new(__var__ti, __var__ti2)
        },
        |ctx, pat| {
            let result = Int___add__::new(pat.tuplendarray_length_node2, pat.tuplendarray_length_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:161
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let ndarray_ndim_node1 = NDArray_ndim::query(&__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = TupleInt_length::new(pat.ndarray_shape_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:163
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let ndarray_to_value_node1 = NDArray_to_value::query(&__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = NDArray_index::new(pat.__var__x, pat.tupleint_empty_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:164
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let value_float_node1 = Value_float::query(&__var__f);
let ndarray_scalar_node3 = NDArray_scalar::query(&value_float_node2);
let value_float_node4 = Value_float::query(&__var__f);
let ndarray_scalar_node6 = NDArray_scalar::query(&value_float_node5);
let ndarray___truediv___node9 = NDArray___truediv__::query(&ndarray_scalar_node7, &ndarray_scalar_node8);
array_api_rulesetPat::new(__var__f)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_float_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:166
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let value_float_node1 = Value_float::query(&__var__f);
let ndarray_scalar_node3 = NDArray_scalar::query(&value_float_node2);
let value_float_node4 = Value_float::query(&__var__f);
let ndarray_scalar_node6 = NDArray_scalar::query(&value_float_node5);
let ndarray___sub___node9 = NDArray___sub__::query(&ndarray_scalar_node7, &ndarray_scalar_node8);
array_api_rulesetPat::new(__var__f)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_float_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:168
    // Rule: rule_168
    MyTx::add_rule(
        "rule_168",
        default_ruleset,
        || {
            let __var__fi1 = Expr::query_leaf();
let float___init___node1 = Float___init__::query(&__var__fi1);
let value_float_node3 = Value_float::query(&float___init___node2);
let ndarray_scalar_node5 = NDArray_scalar::query(&value_float_node4);
let __var__fi2 = Expr::query_leaf();
let float___init___node6 = Float___init__::query(&__var__fi2);
let value_float_node8 = Value_float::query(&float___init___node7);
let ndarray_scalar_node10 = NDArray_scalar::query(&value_float_node9);
let ndarray___gt___node13 = NDArray___gt__::query(&ndarray_scalar_node11, &ndarray_scalar_node12);
rule_168Pat::new(__var__fi1, __var__fi2)
        },
        |ctx, pat| {
            let result = NDArray_scalar::new(pat.value_bool_node2);
ctx.union(pat.rule_168_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:171
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__x = Expr::query_leaf();
let ndarray_t_node1 = NDArray_T::query(&__var__x);
let ndarray_t_node3 = NDArray_T::query(&ndarray_t_node2);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__x);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:174
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__ti = Expr::query_leaf();
let tuplevalue_empty_node1 = TupleValue_EMPTY::query();
let tuplevalue___add___node3 = TupleValue___add__::query(&__var__ti, &tuplevalue_empty_node2);
array_api_rulesetPat::new(__var__ti)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__ti);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:175
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let tuplevalue_empty_node1 = TupleValue_EMPTY::query();
let tuplevalue_length_node3 = TupleValue_length::query(&tuplevalue_empty_node2);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = Int___init__::new(0);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:176
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let tuplevalue___init___node1 = TupleValue___init__::query(&__var__v);
let tuplevalue_length_node3 = TupleValue_length::query(&tuplevalue___init___node2);
array_api_rulesetPat::new(__var__v)
        },
        |ctx, pat| {
            let result = Int___init__::new(1);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:177
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__ti = Expr::query_leaf();
let __var__ti2 = Expr::query_leaf();
let tuplevalue___add___node1 = TupleValue___add__::query(&__var__ti, &__var__ti2);
let tuplevalue_length_node3 = TupleValue_length::query(&tuplevalue___add___node2);
array_api_rulesetPat::new(__var__ti, __var__ti2)
        },
        |ctx, pat| {
            let result = Int___add__::new(pat.tuplevalue_length_node2, pat.tuplevalue_length_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:178
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let tuplevalue___init___node1 = TupleValue___init__::query(&__var__v);
let int___init___node2 = Int___init__::query(0);
let tuplevalue___getitem___node5 = TupleValue___getitem__::query(&tuplevalue___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__v)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__v);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:179
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__v = Expr::query_leaf();
let tuplevalue___init___node1 = TupleValue___init__::query(&__var__v);
let __var__ti = Expr::query_leaf();
let tuplevalue___add___node3 = TupleValue___add__::query(&tuplevalue___init___node2, &__var__ti);
let int___init___node4 = Int___init__::query(0);
let tuplevalue___getitem___node7 = TupleValue___getitem__::query(&tuplevalue___add___node5, &int___init___node6);
array_api_rulesetPat::new(__var__v, __var__ti)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__v);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:180
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__ti = Expr::query_leaf();
let __var__ti2 = Expr::query_leaf();
let tuplevalue___add___node1 = TupleValue___add__::query(&__var__ti, &__var__ti2);
let __var__v = Expr::query_leaf();
let tuplevalue_includes_node3 = TupleValue_includes::query(&tuplevalue___add___node2, &__var__v);
array_api_rulesetPat::new(__var__ti, __var__ti2, __var__v)
        },
        |ctx, pat| {
            let result = Boolean___or__::new(pat.tuplevalue_includes_node2, pat.tuplevalue_includes_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:182
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let value_int_node1 = Value_int::query(&__var__i);
let value_dtype_node3 = Value_dtype::query(&value_int_node2);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = DType_int64::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:183
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let value_float_node1 = Value_float::query(&__var__f);
let value_dtype_node3 = Value_dtype::query(&value_float_node2);
array_api_rulesetPat::new(__var__f)
        },
        |ctx, pat| {
            let result = DType_float64::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:185
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__b = Expr::query_leaf();
let value_bool_node1 = Value_bool::query(&__var__b);
let value_dtype_node3 = Value_dtype::query(&value_bool_node2);
array_api_rulesetPat::new(__var__b)
        },
        |ctx, pat| {
            let result = DType_bool::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:187
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__b = Expr::query_leaf();
let value_bool_node1 = Value_bool::query(&__var__b);
let value_to_bool_node3 = Value_to_bool::query(&value_bool_node2);
array_api_rulesetPat::new(__var__b)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__b);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:189
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let value_int_node1 = Value_int::query(&__var__i);
let value_to_int_node3 = Value_to_int::query(&value_int_node2);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:190
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__b = Expr::query_leaf();
let value_bool_node1 = Value_bool::query(&__var__b);
let value_to_truthy_value_node3 = Value_to_truthy_value::query(&value_bool_node2);
array_api_rulesetPat::new(__var__b)
        },
        |ctx, pat| {
            let result = Value_bool::new(pat.__var__b);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:195
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("integral");
let isdtype_node5 = isdtype::query(&dtype_float32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:196
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("integral");
let isdtype_node5 = isdtype::query(&dtype_float64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:198
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("integral");
let isdtype_node5 = isdtype::query(&dtype_object_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:199
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("integral");
let isdtype_node5 = isdtype::query(&dtype_int64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:201
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("integral");
let isdtype_node5 = isdtype::query(&dtype_int32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:202
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("real floating");
let isdtype_node5 = isdtype::query(&dtype_float32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:203
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("real floating");
let isdtype_node5 = isdtype::query(&dtype_float64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:204
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("real floating");
let isdtype_node5 = isdtype::query(&dtype_object_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:205
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("real floating");
let isdtype_node5 = isdtype::query(&dtype_int64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:206
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("real floating");
let isdtype_node5 = isdtype::query(&dtype_int32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:207
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("complex floating");
let isdtype_node5 = isdtype::query(&dtype_float32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:208
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("complex floating");
let isdtype_node5 = isdtype::query(&dtype_float64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:209
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("complex floating");
let isdtype_node5 = isdtype::query(&dtype_object_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:210
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("complex floating");
let isdtype_node5 = isdtype::query(&dtype_int64_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:211
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let isdtypekind_string_node2 = IsDtypeKind_string::query("complex floating");
let isdtype_node5 = isdtype::query(&dtype_int32_node3, &isdtypekind_string_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:213
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__d = Expr::query_leaf();
let isdtypekind_null_node1 = IsDtypeKind_NULL::query();
let isdtype_node3 = isdtype::query(&__var__d, &isdtypekind_null_node2);
array_api_rulesetPat::new(__var__d)
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:215
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__d = Expr::query_leaf();
let isdtypekind_dtype_node1 = IsDtypeKind_dtype::query(&__var__d);
let isdtype_node3 = isdtype::query(&__var__d, &isdtypekind_dtype_node2);
array_api_rulesetPat::new(__var__d)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:216
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__k1 = Expr::query_leaf();
let isdtypekind_null_node1 = IsDtypeKind_NULL::query();
let isdtypekind___or___node3 = IsDtypeKind___or__::query(&__var__k1, &isdtypekind_null_node2);
array_api_rulesetPat::new(__var__k1)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__k1);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:218
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let dtype_float64_node2 = DType_float64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float64_node3, &dtype_float64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:219
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let dtype_float32_node2 = DType_float32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float64_node3, &dtype_float32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:220
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let dtype_int32_node2 = DType_int32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float64_node3, &dtype_int32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:221
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let dtype_int64_node2 = DType_int64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float64_node3, &dtype_int64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:222
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float64_node1 = DType_float64::query();
let dtype_object_node2 = DType_object::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float64_node3, &dtype_object_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:223
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let dtype_float64_node2 = DType_float64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float32_node3, &dtype_float64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:224
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let dtype_float32_node2 = DType_float32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float32_node3, &dtype_float32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:225
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let dtype_int32_node2 = DType_int32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float32_node3, &dtype_int32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:226
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let dtype_int64_node2 = DType_int64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float32_node3, &dtype_int64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:227
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_float32_node1 = DType_float32::query();
let dtype_object_node2 = DType_object::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_float32_node3, &dtype_object_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:228
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let dtype_float64_node2 = DType_float64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int32_node3, &dtype_float64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:229
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let dtype_float32_node2 = DType_float32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int32_node3, &dtype_float32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:230
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let dtype_int32_node2 = DType_int32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int32_node3, &dtype_int32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:231
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let dtype_int64_node2 = DType_int64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int32_node3, &dtype_int64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:232
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int32_node1 = DType_int32::query();
let dtype_object_node2 = DType_object::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int32_node3, &dtype_object_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:233
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let dtype_float64_node2 = DType_float64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int64_node3, &dtype_float64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:234
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let dtype_float32_node2 = DType_float32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int64_node3, &dtype_float32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:235
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let dtype_int32_node2 = DType_int32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int64_node3, &dtype_int32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:236
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let dtype_int64_node2 = DType_int64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int64_node3, &dtype_int64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:237
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_int64_node1 = DType_int64::query();
let dtype_object_node2 = DType_object::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_int64_node3, &dtype_object_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:238
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let dtype_float64_node2 = DType_float64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_object_node3, &dtype_float64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:239
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let dtype_float32_node2 = DType_float32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_object_node3, &dtype_float32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:240
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let dtype_int32_node2 = DType_int32::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_object_node3, &dtype_int32_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:241
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let dtype_int64_node2 = DType_int64::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_object_node3, &dtype_int64_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:242
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let dtype_object_node1 = DType_object::query();
let dtype_object_node2 = DType_object::query();
let dtype___eq___node5 = DType___eq__::query(&dtype_object_node3, &dtype_object_node4);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:244
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__idx_fn = Expr::query_leaf();
let __var__i = Expr::query_leaf();
let cast_callable__unstablefn_int__int___int___int___lambda_idx_fn__i__idx_fn_i___int_1____node1 = cast_Callable__UnstableFn_Int__Int___Int___Int___lambda_idx_fn__i__idx_fn_i___Int_1___::query(&__var__idx_fn, &__var__i);
array_api_rulesetPat::new(__var__idx_fn, __var__i)
        },
        |ctx, pat| {
            let result = unstable_app::new(pat.__var__idx_fn, pat.int___add___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:245
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__vs = Expr::query_leaf();
let __var__k = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__k);
let index_vec_int_node3 = index_vec_int::query(&__var__vs, &int___init___node2);
array_api_rulesetPat::new(__var__vs, __var__k)
        },
        |ctx, pat| {
            let result = vec_get::new(pat.__var__vs, pat.__var__k);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:246
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let __var__idx_fn = Expr::query_leaf();
let tupleint___init___node1 = TupleInt___init__::query(&__var__i, &__var__idx_fn);
let tupleint_length_node3 = TupleInt_length::query(&tupleint___init___node2);
array_api_rulesetPat::new(__var__i, __var__idx_fn)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:247
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let __var__idx_fn = Expr::query_leaf();
let tupleint___init___node1 = TupleInt___init__::query(&__var__i, &__var__idx_fn);
let __var__i2 = Expr::query_leaf();
let tupleint___getitem___node3 = TupleInt___getitem__::query(&tupleint___init___node2, &__var__i2);
array_api_rulesetPat::new(__var__i, __var__idx_fn, __var__i2)
        },
        |ctx, pat| {
            let result = unstable_app::new(pat.__var__idx_fn, pat.__var__i2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:248
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let int___init___node1 = Int___init__::query(0);
let __var__idx_fn = Expr::query_leaf();
let tupleint___init___node3 = TupleInt___init__::query(&int___init___node2, &__var__idx_fn);
let __var__i = Expr::query_leaf();
let __var__f = Expr::query_leaf();
let tupleint_fold_node5 = TupleInt_fold::query(&tupleint___init___node4, &__var__i, &__var__f);
array_api_rulesetPat::new(__var__idx_fn, __var__i, __var__f)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:249
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let int___init___node1 = Int___init__::query(0);
let __var__idx_fn = Expr::query_leaf();
let tupleint___init___node3 = TupleInt___init__::query(&int___init___node2, &__var__idx_fn);
let __var__b = Expr::query_leaf();
let __var__bool_f = Expr::query_leaf();
let tupleint_fold_boolean_node5 = TupleInt_fold_boolean::query(&tupleint___init___node4, &__var__b, &__var__bool_f);
array_api_rulesetPat::new(__var__idx_fn, __var__b, __var__bool_f)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__b);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:251
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let tupleint_empty_node1 = TupleInt_EMPTY::query();
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TupleInt___init__::new(pat.int___init___node2, pat.unstable_fn_node3);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:252
    // Rule: rule_252
    MyTx::add_rule(
        "rule_252",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let float___init___node1 = Float___init__::query(&__var__f);
let float_abs_node3 = Float_abs::query(&float___init___node2);
rule_252Pat::new(__var__f)
        },
        |ctx, pat| {
            let result = Float___init__::new(pat.__var__f);
ctx.union(pat.rule_252_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:256
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let float_from_int_node3 = Float_from_int::query(&int___init___node2);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = Float_rational::new(pat.bigrat_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:258
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let float___init___node1 = Float___init__::query(&__var__f);
let __var__f2 = Expr::query_leaf();
let float___init___node2 = Float___init__::query(&__var__f2);
let float___add___node5 = Float___add__::query(&float___init___node3, &float___init___node4);
array_api_rulesetPat::new(__var__f, __var__f2)
        },
        |ctx, pat| {
            let result = Float___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:261
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__f = Expr::query_leaf();
let float___init___node1 = Float___init__::query(&__var__f);
let __var__f2 = Expr::query_leaf();
let float___init___node2 = Float___init__::query(&__var__f2);
let float___mul___node5 = Float___mul__::query(&float___init___node3, &float___init___node4);
array_api_rulesetPat::new(__var__f, __var__f2)
        },
        |ctx, pat| {
            let result = Float___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:263
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__r = Expr::query_leaf();
let float_rational_node1 = Float_rational::query(&__var__r);
let __var__r1 = Expr::query_leaf();
let float_rational_node2 = Float_rational::query(&__var__r1);
let float___truediv___node5 = Float___truediv__::query(&float_rational_node3, &float_rational_node4);
array_api_rulesetPat::new(__var__r, __var__r1)
        },
        |ctx, pat| {
            let result = Float_rational::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:264
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__r = Expr::query_leaf();
let float_rational_node1 = Float_rational::query(&__var__r);
let __var__r1 = Expr::query_leaf();
let float_rational_node2 = Float_rational::query(&__var__r1);
let float___add___node5 = Float___add__::query(&float_rational_node3, &float_rational_node4);
array_api_rulesetPat::new(__var__r, __var__r1)
        },
        |ctx, pat| {
            let result = Float_rational::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:265
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__r = Expr::query_leaf();
let float_rational_node1 = Float_rational::query(&__var__r);
let __var__r1 = Expr::query_leaf();
let float_rational_node2 = Float_rational::query(&__var__r1);
let float___mul___node5 = Float___mul__::query(&float_rational_node3, &float_rational_node4);
array_api_rulesetPat::new(__var__r, __var__r1)
        },
        |ctx, pat| {
            let result = Float_rational::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:266
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let int___init___node2 = Int___init__::query(&__var__i);
let int___eq___node5 = Int___eq__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:268
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let int___init___node2 = Int___init__::query(&__var__i);
let int___ge___node5 = Int___ge__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:269
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let int___init___node2 = Int___init__::query(&__var__i);
let int___lt___node5 = Int___lt__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:271
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let int___init___node2 = Int___init__::query(&__var__i);
let int___gt___node5 = Int___gt__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:272
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___add___node5 = Int___add__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:273
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___mul___node5 = Int___mul__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:275
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___truediv___node5 = Int___truediv__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:277
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___mod___node5 = Int___mod__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:279
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___and___node5 = Int___and__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:281
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___or___node5 = Int___or__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:283
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___xor___node5 = Int___xor__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.__node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:285
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___lshift___node5 = Int___lshift__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:287
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let __var__j = Expr::query_leaf();
let int___init___node2 = Int___init__::query(&__var__j);
let int___rshift___node5 = Int___rshift__::query(&int___init___node3, &int___init___node4);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.___node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:289
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let __var__i = Expr::query_leaf();
let int___init___node1 = Int___init__::query(&__var__i);
let int___invert___node3 = Int___invert__::query(&int___init___node2);
array_api_rulesetPat::new(__var__i)
        },
        |ctx, pat| {
            let result = Int___init__::new(pat.not_i64_node2);
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:290
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let true_node1 = TRUE::query();
let __var__o = Expr::query_leaf();
let __var__b = Expr::query_leaf();
let int_if__node3 = Int_if_::query(&true_node2, &__var__o, &__var__b);
array_api_rulesetPat::new(__var__o, __var__b)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__o);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:291
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let false_node1 = FALSE::query();
let __var__o = Expr::query_leaf();
let __var__b = Expr::query_leaf();
let int_if__node3 = Int_if_::query(&false_node2, &__var__o, &__var__b);
array_api_rulesetPat::new(__var__o, __var__b)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__b);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:294
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let true_node1 = TRUE::query();
let __var__x = Expr::query_leaf();
let boolean___or___node3 = Boolean___or__::query(&true_node2, &__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:295
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let false_node1 = FALSE::query();
let __var__x = Expr::query_leaf();
let boolean___or___node3 = Boolean___or__::query(&false_node2, &__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__x);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:297
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let true_node1 = TRUE::query();
let __var__x = Expr::query_leaf();
let boolean___and___node3 = Boolean___and__::query(&true_node2, &__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__x);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:298
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let false_node1 = FALSE::query();
let __var__x = Expr::query_leaf();
let boolean___and___node3 = Boolean___and__::query(&false_node2, &__var__x);
array_api_rulesetPat::new(__var__x)
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:300
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let true_node1 = TRUE::query();
let __var__i = Expr::query_leaf();
let __var__j = Expr::query_leaf();
let boolean_if_int_node3 = Boolean_if_int::query(&true_node2, &__var__i, &__var__j);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__i);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:301
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let false_node1 = FALSE::query();
let __var__i = Expr::query_leaf();
let __var__j = Expr::query_leaf();
let boolean_if_int_node3 = Boolean_if_int::query(&false_node2, &__var__i, &__var__j);
array_api_rulesetPat::new(__var__i, __var__j)
        },
        |ctx, pat| {
            ctx.union(pat.array_api_ruleset_node1, pat.__var__j);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:303
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let true_node1 = TRUE::query();
let boolean___invert___node3 = Boolean___invert__::query(&true_node2);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = FALSE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:304
    // Rule: array_api_ruleset
    MyTx::add_rule(
        "array_api_ruleset",
        default_ruleset,
        || {
            let false_node1 = FALSE::query();
let boolean___invert___node3 = Boolean___invert__::query(&false_node2);
array_api_rulesetPat::new()
        },
        |ctx, pat| {
            let result = TRUE::new();
ctx.union(pat.array_api_ruleset_node1, result);
        },
    );
    
    // Source: examples/stresstest_large_expr.egg:336
    current_expr.pull();
    // Source: examples/stresstest_large_expr.egg:336
    let default_ruleset = MyTx::new_ruleset("default_ruleset");
    // Source: examples/stresstest_large_expr.egg:336
    MyTx::run_ruleset(MyTx, RunConfig::Sat);
    println!("Eggplant program executed successfully!");
}
