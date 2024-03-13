package com.example.utils;

import com.example.entity.RestBean;
import org.springframework.stereotype.Component;

import java.util.function.Supplier;

@Component
public class ControllerUtils {
    /**
     * 针对于返回值为String作为错误信息的方法进行统一处理
     *用supplier的get取得操作的返回值
     * @param action 具体操作
     * @param <T>    响应结果类型
     * @return 响应结果
     */
    public <T> RestBean<T> messageHandle(Supplier<String> action) {
        String message = action.get();
        if (message == null)
            return RestBean.success();
        else
            return RestBean.failure(400, message);
    }
}
