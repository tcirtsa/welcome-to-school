package com.example.controller;

import com.example.entity.RestBean;
import com.example.service.ImageService;
import io.minio.errors.ErrorResponseException;
import jakarta.annotation.Resource;
import jakarta.servlet.ServletOutputStream;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import lombok.extern.slf4j.Slf4j;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.io.IOException;

@RestController
@Slf4j
public class ObjectController {

    @Resource
    ImageService service;

    /**
     * 获取在minio里的图片
     *
     * @param request
     * @param response
     * @throws Exception
     */
    @GetMapping("/images/**")
    public void imageFetch(HttpServletRequest request, HttpServletResponse response) throws Exception {
        response.setHeader("Content-Type", "image/jpg");
        this.fetchImage(request,response);
    }
    private void fetchImage(HttpServletRequest request, HttpServletResponse response) throws Exception {
        //getServletPath会有/images/这个前缀要去掉
        String imagePath=request.getServletPath().substring(7);
        //ServletOutputStream与HttpServletResponse有关，直接输出到这，相对应的request有InputStream
        ServletOutputStream stream= response.getOutputStream();
        //如果图片路径果断就判断为恶意请求直接设置404
        if(imagePath.length()<=13){
            response.setStatus(404);
            stream.println(RestBean.failure(404,"Not Found").toString());
        }else {
            try {
                //将文件路径写入输出流
                service.fetchAvatarFromMinio(stream,imagePath);
                //设置取得的图片最大缓存时间
                response.setHeader("Cache-Control","max-age=2592000");
            }catch (ErrorResponseException e){
                if(e.response().code() == 404){
                    response.setStatus(404);
                    stream.println(RestBean.failure(404,"Not Found").toString());
                }else {
                    log.error("从Minio获取图片失败"+e.getMessage(),e);
                }
            }
        }
    }

}
