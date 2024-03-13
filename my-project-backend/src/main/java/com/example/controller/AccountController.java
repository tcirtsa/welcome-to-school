package com.example.controller;

import com.example.entity.RestBean;
import com.example.entity.dto.Account;
import com.example.entity.dto.AccountDetails;
import com.example.entity.vo.request.ChangePasswordVO;
import com.example.entity.vo.request.DetailsSaveVO;
import com.example.entity.vo.request.ModifyEmailVO;
import com.example.entity.vo.request.SavePrivacyVO;
import com.example.entity.vo.response.AccountDetailsVO;
import com.example.entity.vo.response.AccountPrivacyVO;
import com.example.entity.vo.response.AccountVO;
import com.example.service.AccountDetailsService;
import com.example.service.AccountService;
import com.example.utils.Const;
import com.example.utils.ControllerUtils;
import jakarta.annotation.Resource;
import jakarta.validation.Valid;
import org.springframework.web.bind.annotation.*;

import java.util.Optional;

@RestController
@RequestMapping("/api/user")
public class AccountController {
    @Resource
    AccountService service;

    @Resource
    AccountDetailsService detailsService;

    @Resource
    ControllerUtils utils;

    /**
     * 查询用户基本信息
     *
     * @param id 用户id
     * @return 用户vo
     */
    @GetMapping("/info")
    public RestBean<AccountVO> info(@RequestAttribute(Const.ATTR_USER_ID) int id) {
        Account account = service.findAccountById(id);
        return RestBean.success(account.asViewObject(AccountVO.class));
    }

    /**
     * 查询用户隐私设置信息
     * @param id 用户id
     * @return 隐私vo
     */
    @GetMapping("/details")
    public RestBean<AccountDetailsVO> details(@RequestAttribute(Const.ATTR_USER_ID)int id){
            AccountDetails details= Optional
                    .ofNullable(detailsService.findAccountDetails(id))
                    .orElseGet(AccountDetails::new);
        return RestBean.success(details.asViewObject(AccountDetailsVO.class));
    }

    /**
     * 修改邮箱
     * @param id 用户id
     * @param vo 验证码和邮箱vo
     * @return 状态码
     */
    @PostMapping("/modify-email")
    public RestBean<Void> modifyEmail(@RequestAttribute(Const.ATTR_USER_ID)int id,
                                      @RequestBody @Valid ModifyEmailVO vo){
        String result=service.modifyEmail(id,vo);
        return result==null ?RestBean.success() : RestBean.failure(400,result);
    }

    /**
     * 保存用户信息修改
     * @param id 用户id
     * @param vo 用户信息修改
     * @return 状态码
     */
    @PostMapping("/save-details")
        public RestBean<Void> saveDetails(@RequestAttribute(Const.ATTR_USER_ID)int id,
                                          @RequestBody @Valid DetailsSaveVO vo){
        boolean success=detailsService.saveAccountDetails(id,vo);
        return success ? RestBean.success() : RestBean.failure(400,"用户名已被注册");
    }

    /**
     * 修改密码接口
     * @param id 用户id
     * @param vo 新旧密码vo
     * @return 处理信息
     */
    @PostMapping("/change-password")
    public RestBean<Void> changePassword(@RequestAttribute(Const.ATTR_USER_ID)int id,
                                      @RequestBody @Valid ChangePasswordVO vo){

        return utils.messageHandle(() -> service.changePassword(id, vo));
    }

}
