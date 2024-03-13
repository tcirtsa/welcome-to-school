package com.example.entity.vo.response;

import lombok.Data;

import java.util.Date;

@Data
public class NotificationVO {
    Integer id;
    String content;
    String title;
    String type;
    String url;
    Date time;
}
