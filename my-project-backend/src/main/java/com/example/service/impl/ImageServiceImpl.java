package com.example.service.impl;

import com.baomidou.mybatisplus.core.toolkit.Wrappers;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.example.entity.dto.Account;
import com.example.entity.dto.ImageStore;
import com.example.mapper.AccountMapper;
import com.example.mapper.ImageStoreMapper;
import com.example.service.ImageService;
import com.example.utils.Const;
import com.example.utils.FlowUtils;
import io.minio.*;
import jakarta.annotation.Resource;
import lombok.extern.slf4j.Slf4j;
import org.apache.tomcat.util.http.fileupload.IOUtils;
import org.springframework.stereotype.Service;
import org.springframework.web.multipart.MultipartFile;

import java.awt.*;
import java.io.IOException;
import java.io.OutputStream;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.UUID;
@Slf4j
@Service
public class ImageServiceImpl extends ServiceImpl<ImageStoreMapper, ImageStore> implements ImageService {

    @Resource
    MinioClient client;
    @Resource
    FlowUtils utils;
    @Resource
    AccountMapper mapper;
    //创建一个时间类型格式
    SimpleDateFormat dateFormat = new SimpleDateFormat("yyyyMMdd");

    /**
     * 上传头像
     *
     * @param file
     * @param id
     * @return 头像路径
     * @throws IOException
     */
    @Override
    public String uploadAvatar(MultipartFile file, int id) throws IOException {
        //生成每个图片的唯一标识并去掉—便于查看
        String imageName= UUID.randomUUID().toString().replace("-","");
        imageName="/avatar/"+imageName;
        //建立与wall的连接
        PutObjectArgs args= PutObjectArgs.builder()
                .bucket("wall")
                .stream(file.getInputStream(), file.getSize(), -1)
                .object(imageName)
                .build();
        try {
            client.putObject(args);
            String avatar = mapper.selectById(id).getAvatar();
            //上传完新的头像就删除原本的头像
            this.deleteAvatar(avatar);
            if(mapper.update(null,Wrappers.<Account>update().eq("id",id).set("avatar",imageName))>0){
                return imageName;
            }else
                return null;
        }catch (Exception e){
            log.error("上传图片发生错误：" + e.getMessage());
            return null;
        }
    }

    /**
     * 上传图片
     * @param file 文件
     * @param id 用户id
     * @return 图片路径
     * @throws IOException
     */
    @Override
    public String uploadImage(MultipartFile file, int id) throws IOException {
        String key = Const.FORUM_IMAGE_COUNTER + id;
        if (!utils.limitPeriodCounterCheck(key, 20, 3600))
            return null;
        String imageName = UUID.randomUUID().toString().replace("-", "");
        Date date = new Date();
        //给路径加上上传时间(其实是处理这一段代码的时间)
        imageName = "/cache/" + dateFormat.format(date) + "/" + imageName;
        PutObjectArgs args = PutObjectArgs.builder()
                .bucket("wall")
                .stream(file.getInputStream(), file.getSize(), -1)
                .object(imageName)
                .build();
        try {
            client.putObject(args);
            if (this.save(new ImageStore(id, imageName, date))) {
                return imageName;
            } else {
                return null;
            }
        } catch (Exception e) {
            log.error("上传图片发生错误：" + e.getMessage(), e);
            return null;
        }
    }

    /**
     * 获取头像响应体并写入输出流
     * @param stream 输出流
     * @param imagePath 图片路径
     * @throws Exception
     */
    @Override
    public void fetchAvatarFromMinio(OutputStream stream, String imagePath) throws Exception {
        GetObjectArgs args= GetObjectArgs.builder()
                .bucket("wall")
                .object(imagePath)
                .build();
        //将得到的响应体写入输出流
        GetObjectResponse response = client.getObject(args);
        IOUtils.copy(response,stream);
    }

    /**
     * 删除头像
     * @param avatar
     * @throws Exception
     */
    private void deleteAvatar(String avatar) throws Exception {
        if (avatar == null || avatar.isEmpty()) {
            return;
        }
        RemoveObjectArgs remove = RemoveObjectArgs.builder()
                .bucket("wall")
                .object(avatar)
                .build();
        client.removeObject(remove);
    }
}
