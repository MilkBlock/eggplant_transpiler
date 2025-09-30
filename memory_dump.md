# Eggplant Transpiler Memory Dump

## 当前状态总结

### 修复的问题
1. **PatternVars 生成修复**:
   - 之前: PatternVars 错误地包含 basic type 变量
   - 现在: PatternVars 只包含 complex type 变量和 constructor nodes
   - Basic type 变量通过 constructor instance fields 访问

2. **Action 生成修复**:
   - 现在正确使用 `pat.conste_node1.arg_i64_00` 访问 basic type 字段
   - 通过 `ctx.devalue()` 将 Value 类型转换为基本类型

3. **重复 Constructor Node 生成修复**:
   - 之前: 同一构造器调用中的变量会生成多个 constructor nodes
   - 现在: 同一构造器调用中的所有变量共享一个 constructor node
   - 变量通过共享 constructor node 的字段访问基本类型值

### 技术实现

#### PatternVars 生成逻辑
```rust
// 创建 pattern variables struct - 包含 constructor nodes 用于 basic type 访问
let mut pattern_vars_variables = variables.clone();

// 添加 constructor nodes 到 pattern variables 用于 basic type 字段访问
for (var_name, (constructor_name, node_name, _)) in &variable_constructors {
    if !pattern_vars_variables.iter().any(|v| v.name == *node_name) {
        pattern_vars_variables.push(PatternVariable {
            name: node_name.clone(),
            var_type: constructor_name.clone(),
        });
    }
}
```

#### 共享 Constructor Node 生成逻辑
```rust
// 生成共享的 constructor node 名称 (同一构造器调用中的所有变量共享)
*node_counter += 1;
let constructor_node_name = format!(
    "{}_node{}",
    normalize_identifier(&func_name.to_snake_case()),
    node_counter
);

for (index, arg) in args.iter().enumerate() {
    if let Expr::Var(_, var_name) = arg {
        // 记录变量与共享 constructor node 的关联
        variable_constructors.insert(var_name.clone(), (func_name.clone(), constructor_node_name.clone(), index));
    }
}
```

#### 变量类型推断
- **Complex Types**: 自定义类型 (Expr, Const, Div)
- **Basic Types**: 基本类型 (i64, f64, bool, String, ())
- **Constructor Nodes**: 用于访问 basic type 字段的查询节点

#### Query 方法选择
- `query_leaf()`: 用于没有参数的叶节点构造器
- `query()`: 用于有参数的内部节点构造器

### 测试结果验证

#### Test 1: (DivE a b) - 修复后
- **PatternVars**: `dive_node1` (共享 constructor node)
- **Action**: `ctx.devalue(pat.dive_node1.arg_i64_10) / ctx.devalue(pat.dive_node1.arg_i64_11)`
- **说明**: 同一构造器调用中的变量 `a` 和 `b` 共享一个 constructor node

#### Test 2: (Div a (Const 0)) - 修复后
- **PatternVars**: `a` (complex type), `div_node1` (constructor node)
- **Action**: `Const::new(0)`
- **说明**: 不同构造器调用生成各自的 constructor nodes

### 关键函数

1. `extract_variables_with_types_and_context()` - 提取变量并构建查询模式树
2. `generate_pattern_query_with_context()` - 生成带上下文的模式查询
3. `generate_rewrite_action_with_context()` - 生成带上下文的改写动作
4. `is_basic_type()` - 检查是否为基本类型
5. `is_leaf_constructor()` - 检查是否为叶节点构造器

### 当前工作目录结构
```
/eggplant_transpiler/
├── src/
│   ├── eggplant.rs      # 核心 transpiler 实现
│   ├── ast/             # AST 定义和解析
│   └── codegen.rs       # 代码生成
├── examples/            # 示例程序
├── test_transpiler.rs   # 测试程序
├── test_devalue_zero.rs # devalue 测试
└── memory_dump.md       # 本文件
```

### 下一步工作
- 清理未使用的代码和警告
- 添加更多测试用例
- 优化类型推断算法
- 支持更多 egglog 命令

### 注意事项
- PatternVars 应该包含所有需要在 action 中访问的节点
- Basic type 变量通过 constructor instance fields 访问
- Complex type 变量直接通过 pattern struct 访问
- 使用 `ctx.devalue()` 将 Value 类型转换为基本类型
- 同一构造器调用中的所有变量共享一个 constructor node