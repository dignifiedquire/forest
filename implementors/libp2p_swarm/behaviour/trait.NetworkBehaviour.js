(function() {var implementors = {};
implementors["forest_libp2p"] = [{"text":"impl&lt;TSubstream:&nbsp;<a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static, TSubstream1&gt; NetworkBehaviour for <a class=\"struct\" href=\"forest_libp2p/struct.ForestBehaviour.html\" title=\"struct forest_libp2p::ForestBehaviour\">ForestBehaviour</a>&lt;TSubstream&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Gossipsub&lt;TSubstream&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Gossipsub&lt;TSubstream&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Gossipsub&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler: ProtocolsHandler&lt;Substream = TSubstream1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Gossipsub&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::InboundProtocol: InboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Gossipsub&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::OutboundProtocol: OutboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Mdns&lt;TSubstream&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Mdns&lt;TSubstream&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Mdns&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler: ProtocolsHandler&lt;Substream = TSubstream1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Mdns&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::InboundProtocol: InboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Mdns&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::OutboundProtocol: OutboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ping&lt;TSubstream&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Ping&lt;TSubstream&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Ping&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler: ProtocolsHandler&lt;Substream = TSubstream1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Ping&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::InboundProtocol: InboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Ping&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::OutboundProtocol: OutboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Identify&lt;TSubstream&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Identify&lt;TSubstream&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;Identify&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler: ProtocolsHandler&lt;Substream = TSubstream1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Identify&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::InboundProtocol: InboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;Identify&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::OutboundProtocol: OutboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt;: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;<a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt; as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;<a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler: ProtocolsHandler&lt;Substream = TSubstream1&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;<a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::InboundProtocol: InboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;&lt;<a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt; as NetworkBehaviour&gt;::ProtocolsHandler as IntoProtocolsHandler&gt;::Handler as ProtocolsHandler&gt;::OutboundProtocol: OutboundUpgrade&lt;Negotiated&lt;TSubstream1&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;TSubstream1: <a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;TSubstream1: <a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;TSubstream1: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":false,"types":["forest_libp2p::behaviour::ForestBehaviour"]},{"text":"impl&lt;TSubstream&gt; NetworkBehaviour for <a class=\"struct\" href=\"forest_libp2p/rpc/struct.RPC.html\" title=\"struct forest_libp2p::rpc::RPC\">RPC</a>&lt;TSubstream&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;TSubstream: <a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncRead.html\" title=\"trait futures_io::if_std::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://docs.rs/futures-io/0.3.0/futures_io/if_std/trait.AsyncWrite.html\" title=\"trait futures_io::if_std::AsyncWrite\">AsyncWrite</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>","synthetic":false,"types":["forest_libp2p::rpc::behaviour::RPC"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()