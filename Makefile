# linked at https://developer.atlassian.com/cloud/bitbucket/rest/
BITBUCKET_API_VERSION=2.300.163

.PHONY: all
all: bbapi

bitbucket-openapi-$(BITBUCKET_API_VERSION).json:
	curl --output bitbucket-openapi-$(BITBUCKET_API_VERSION).json \
		https://dac-static.atlassian.com/cloud/bitbucket/swagger.v3.json?_v=$(BITBUCKET_API_VERSION)

bitbucket-openapi-patched-$(BITBUCKET_API_VERSION).json: bitbucket-openapi-$(BITBUCKET_API_VERSION).json openapi-patches.yaml
	python3 scripts/patch_openapi.py \
		--input bitbucket-openapi-$(BITBUCKET_API_VERSION).json \
		--patches openapi-patches.yaml \
		--output bitbucket-openapi-patched-$(BITBUCKET_API_VERSION).json

bitbucket-openapi.yaml: bitbucket-openapi-patched-$(BITBUCKET_API_VERSION).json
	yq -p json -o yaml bitbucket-openapi-patched-$(BITBUCKET_API_VERSION).json > bitbucket-openapi.yaml

bbapi: bitbucket-openapi.yaml
	openapi-generator-cli generate \
	--input-spec bitbucket-openapi.yaml \
	--generator-name rust \
	--additional-properties=packageName=bbapi \
	--additional-properties=packageVersion=$(BITBUCKET_API_VERSION) \
	--output bbapi/
