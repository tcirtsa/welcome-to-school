package com.example.entity.dto;

import com.baomidou.mybatisplus.annotation.IdType;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import com.example.entity.BaseData;
import lombok.Data;

import java.lang.reflect.Field;
import java.util.LinkedList;
import java.util.List;

@Data
@TableName("db_account_privacy")
public class AccountPrivacy implements BaseData {
    @TableId(type = IdType.AUTO)
    final Integer id;
    boolean phone=true;
    boolean qq=true;
    boolean wx=true;
    boolean email=true;
    boolean gender=true;

    //返回隐藏字段的字符串数组
    public String[] hiddenFields() {
        List<String> strings = new LinkedList<>();
        //取出字段
        Field[] fields = this.getClass().getDeclaredFields();
        for (Field field : fields) {
            try {
                //如果是布尔类型并且值为false就加入数组
                if (field.getType().equals(boolean.class) && !field.getBoolean(this)) {
                    strings.add(field.getName());
                }
            } catch (Exception e) {
            }
        }
        //传入类型的generator就行
        return strings.toArray(String[]::new);
    }
}
