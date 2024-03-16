package com.example.entity.vo.request;

import jakarta.validation.constraints.Pattern;
import lombok.Data;

@Data
public class SavePrivacyVO {
    @Pattern(regexp = "(phone|email|qq|wx|gender)")
    String type;
    boolean status;
}
