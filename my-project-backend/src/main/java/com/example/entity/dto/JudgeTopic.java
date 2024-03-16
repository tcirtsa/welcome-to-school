package com.example.entity.dto;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import lombok.Data;

import java.util.Date;

@Data
@TableName("db_judge_topic")
public class JudgeTopic {
    @TableId(type = IdType.AUTO)
    Integer id;
    String title;
    String content;
    Integer uid;
    Integer type;
    Date time;

}
