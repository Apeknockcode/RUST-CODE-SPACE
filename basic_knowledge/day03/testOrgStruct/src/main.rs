fn main() {
    // TODO ; 测试组织结构
    /*
     *  Rust 社区倾向于根据测试的两个主要分类来考虑问题：单元测试（unit tests）与 集成测试（integration tests）
     *  单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口
     *  集成测试对于你的库来说则完全是外部的
     * 
     *  从独立和整体的角度编写这两类测试都是非常重要的。
     * */ 


    //  TODO : 单元测试
    /*
     * 
     * 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期
     * 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。
     * 
     * */ 

    //  TODO : 测试模块和 #[cfg(test)]
    /*
     *  
     * */ 


    //  TODO : 测试私有函数
    // Rust 的私有性规则确实允许你测试私有函数


    // TODO : 集成测试
    /*
     * 在 Rust 中，集成测试对于你需要测试的库来说完全是外部的
     * 集成测试的目的是测试库的多个部分能否一起正常工作.
     * */ 

    //  TODO : 测试私有函数
    

}
