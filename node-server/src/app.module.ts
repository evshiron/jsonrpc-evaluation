import { MiddlewareConsumer, Module, NestModule } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import * as jayson from 'jayson';

@Module({
  imports: [],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule implements NestModule {
  configure(consumer: MiddlewareConsumer) {
    const callServer = new jayson.Server({
      hello(args, callback) {
        console.log('call hello', args, 'return world');
        callback(null, 'world');
      },
    });

    consumer.apply(callServer.middleware()).forRoutes('/rpc');
  }
}
