package com.example.entity.vo.response;

import lombok.Data;

import java.util.Date;

@Data
public class CommentVO {
    int id;
    Date time;
    String content;
    String quote;
    User user;

    @Data
    public static class User {
        Integer id;
        String username;
        String avatar;
        String email;
        String qq;
        String wx;
        Boolean gender;


    }
}
