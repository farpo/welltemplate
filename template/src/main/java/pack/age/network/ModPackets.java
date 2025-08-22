package ``MOD_GROUP``.network;

import ``MOD_GROUP``.``ENTRYPOINT_NAME``;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;
import net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry;
import net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking;
import net.minecraft.network.RegistryByteBuf;
import net.minecraft.network.codec.PacketCodec;
import net.minecraft.network.packet.CustomPayload;

public class ``PACKET_PREFIX``Packets {

    public static void init(){
        initC2S();
        initS2C();
    }
    private static void initS2C(){
    }
    private static void initC2S(){
    }
    private static <T extends ReceiverPacket<ClientPlayNetworking.Context>> void registerClientReceiverPacket(CustomPayload.Id<T> packetId, PacketCodec<? super RegistryByteBuf, T> packetCodec){
        PayloadTypeRegistry.playS2C().register(packetId, packetCodec);
        ClientPlayNetworking.registerGlobalReceiver(packetId, ReceiverPacket::receive);
    }
    private static <T extends ReceiverPacket<ServerPlayNetworking.Context>> void registerServerReceiverPacket(CustomPayload.Id<T> packetId, PacketCodec<? super RegistryByteBuf, T> packetCodec){
        PayloadTypeRegistry.playC2S().register(packetId, packetCodec);
        ServerPlayNetworking.registerGlobalReceiver(packetId, ReceiverPacket::receive);
    }
}
