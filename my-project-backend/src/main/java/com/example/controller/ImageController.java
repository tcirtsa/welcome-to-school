package com.example.controller;

import com.example.entity.RestBean;
import com.example.service.ImageService;
import com.example.utils.Const;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;
import java.io.IOException;

@Slf4j
@RestController
@RequestMapping("/api/image")
public class ImageController {

    @Resource
    ImageService service;

    /**
     * 上传图片到minio
     *
     * @param file 图片
     * @param id   用户id
     * @return 状态码
     * @throws IOException
     */
    @PostMapping("/cache")
    public RestBean<String> uploadImage(@RequestParam("file") MultipartFile file,
                                        @RequestAttribute(Const.ATTR_USER_ID) int id) throws IOException {
        if (file.getSize() > 5 * 1024 * 1024) {
            return RestBean.failure(400, "图片大小不能超过5MB");
        }
        log.info("正在上传图片");
        String url = service.uploadImage(file, id);
        if (url != null) {
            log.info("上传图片成功");
            return RestBean.success(url);
        } else {
            return RestBean.failure(400, "图片上传失败");
        }

    }

    /**
     * 上传头像到minio
     * @param file 头像图片
     * @param id 用户id
     * @return 状态码
     * @throws IOException
     */
    @PostMapping("/avatar")
    public RestBean<String>  uploadAvatar(@RequestParam("file")MultipartFile file,
                                          @RequestAttribute(Const.ATTR_USER_ID) int id) throws IOException {
        if(file.getSize() > 256 * 1024){
            return RestBean.failure(400, "头像大小不能超过256KB");
        }
        log.info("正在上传头像");
        String url = service.uploadAvatar(file, id);
        if(url != null){
            log.info("头像上传成功");
            return RestBean.success();
        }else {
            return RestBean.failure(400,"头像上传失败");
        }
    }
}
