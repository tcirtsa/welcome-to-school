package com.example.entity.vo.request;

import com.alibaba.fastjson2.JSONObject;
import jakarta.validation.constraints.Max;
import jakarta.validation.constraints.Min;
import lombok.Data;

@Data
public class TopicUpdateVO {
    @Min(0)
    int id;
    @Max(5)
    @Min(1)
    int type;
    String title;
    JSONObject content;
}
