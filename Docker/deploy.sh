#!/bin/bash
# Copyright (c) 2022 Petruknisme
# 
# This software is released under the MIT License.
# https://opensource.org/licenses/MIT

docker image build -t petruknisme/spring-rce-shell ./ && docker container run -it --publish 8080:8080 petruknisme/spring-rce-shell