use serde_json::json;

pub fn generate_main_task_payload() -> serde_json::Value {
    let payload = json!({
        "searches": [
            {
                "collection": "nft_collections",
                "q": "*",
                "query_by": "name,slugDisplay,acronym"
            }
        ]
    });

    payload
}

pub fn generate_selected_collections_payload(collection_slug: &str, cursor: &str) -> serde_json::Value {
    return json!([{
        "operationName": "CollectionMintsV2",
        "variables": {
            "slug": collection_slug,
            "sortBy": "ListingPriceAsc",
            "limit": 50,
            "filters": {
                "rarities": null,
                "traits": null,
                "traitCount": null,
                "nameFilter": null,
                "inscripOrderFilters": null,
                "inscripFilters": null,
                "ownerFilter": null,
                "onlyListings": true,
                "listingSources": ["TENSORSWAP", "TCOMP", "HYPERSPACE", "MAGICEDEN_V2", "SOLANART"],
                "listingPrices": null
            },
            "cursor": cursor
        },
        "query": r#"query CollectionMintsV2($slug: String!, $sortBy: CollectionMintsSortBy!, $filters: CollectionMintsFilters, $cursor: String, $limit: Int) {
          collectionMintsV2(
            slug: $slug,
            sortBy: $sortBy,
            filters: $filters,
            cursor: $cursor,
            limit: $limit
          ) {
            mints {
              ...MintWithTx
              __typename
            },
            page {
              endCursor,
              hasMore,
              __typename
            },
            __typename
          }
        }

        fragment MintWithTx on MintWithTx {
          mint {
            ...MintV2
            __typename
          },
          tx {
            ...ReducedParsedTx
            __typename
          },
          __typename
        }

        fragment MintV2 on MintV2 {
          onchainId,
          slug,
          compressed,
          owner,
          name,
          imageUri,
          animationUri,
          metadataUri,
          metadataFetchedAt,
          sellRoyaltyFeeBPS,
          tokenStandard,
          tokenEdition,
          attributes {
            trait_type,
            value,
            __typename
          },
          lastSale {
            price,
            txAt,
            __typename
          },
          accState,
          hidden,
          rarityRankHrtt,
          rarityRankStat,
          rarityRankTeam,
          rarityRankTn,
          inscription {
            ...InscriptionData
            __typename
          },
          tokenProgram,
          metadataProgram,
          __typename
        }

        fragment InscriptionData on InscriptionData {
          inscription,
          inscriptionData,
          immutable,
          order,
          spl20 {
            p,
            tick,
            amt,
            __typename
          },
          __typename
        }

        fragment ReducedParsedTx on ParsedTransaction {
          source,
          txKey,
          txId,
          txType,
          grossAmount,
          sellerId,
          buyerId,
          txAt,
          blockNumber,
          txMetadata {
            auctionHouse,
            urlId,
            sellerRef,
            tokenAcc,
            __typename
          },
          poolOnchainId,
          __typename
        }"#,
    }]);
}

pub fn generate_recent_transactions_payload(slug: &str) -> serde_json::Value {
  json!([{
      "operationName": "CollectionMintsV2",
      "variables": {
          "slug": slug,
          "sortBy": "ListingPriceAsc",
          "limit": 50,
          "filters": {
              "rarities": null,
              "traits": null,
              "traitCount": null,
              "nameFilter": null,
              "inscripOrderFilters": null,
              "inscripFilters": null,
              "ownerFilter": null,
              "onlyListings": true,
              "listingSources": ["TENSORSWAP", "TCOMP", "HYPERSPACE", "MAGICEDEN_V2", "SOLANART"],
              "listingPrices": null
          }
      },
      "query": "query CollectionMintsV2($slug: String!, $sortBy: CollectionMintsSortBy!, $filters: CollectionMintsFilters, $cursor: String, $limit: Int) {\n  collectionMintsV2(\n    slug: $slug\n    sortBy: $sortBy\n    filters: $filters\n    cursor: $cursor\n    limit: $limit\n  ) {\n    mints {\n      ...MintWithTx\n      __typename\n    }\n    page {\n      endCursor\n      hasMore\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment MintWithTx on MintWithTx {\n  mint {\n    ...MintV2\n    __typename\n  }\n  tx {\n    ...ReducedParsedTx\n    __typename\n  }\n  __typename\n}\n\nfragment MintV2 on MintV2 {\n  onchainId\n  slug\n  compressed\n  owner\n  name\n  imageUri\n  animationUri\n  metadataUri\n  metadataFetchedAt\n  sellRoyaltyFeeBPS\n  tokenStandard\n  tokenEdition\n  attributes {\n    trait_type\n    value\n    __typename\n  }\n  lastSale {\n    price\n    txAt\n    __typename\n  }\n  accState\n  hidden\n  rarityRankHrtt\n  rarityRankStat\n  rarityRankTeam\n  rarityRankTn\n  inscription {\n    ...InscriptionData\n    __typename\n  }\n  tokenProgram\n  metadataProgram\n  __typename\n}\n\nfragment InscriptionData on InscriptionData {\n  inscription\n  inscriptionData\n  immutable\n  order\n  spl20 {\n    p\n    tick\n    amt\n    __typename\n  }\n  __typename\n}\n\nfragment ReducedParsedTx on ParsedTransaction {\n  source\n  txKey\n  txId\n  txType\n  grossAmount\n  sellerId\n  buyerId\n  txAt\n  blockNumber\n  txMetadata {\n    auctionHouse\n    urlId\n    sellerRef\n    tokenAcc\n    __typename\n  }\n  poolOnchainId\n  __typename\n}"
  }])
}
